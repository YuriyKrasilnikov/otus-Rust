mod models;
mod token_output_stream;
mod utils;

#[cfg(feature = "mkl")]
extern crate intel_mkl_src;

#[cfg(feature = "accelerate")]
extern crate accelerate_src;

use anyhow::{Error as E, Result};
use candle_core::{DType, Device, Tensor};
use candle_nn::VarBuilder;
use candle_transformers::generation::LogitsProcessor;
use clap::Parser;
use models::{Config, Model};
use token_output_stream::TokenOutputStream;
use tokenizers::Tokenizer;

use log::info;

use tracing_chrome::ChromeLayerBuilder;
use tracing_subscriber::prelude::*;

struct TextGeneration {
    model: Model,
    device: Device,
    tokenizer: TokenOutputStream,
    logits_processor: LogitsProcessor,
    repeat_penalty: f32,
    repeat_last_n: usize,
}

impl TextGeneration {
    #[allow(clippy::too_many_arguments)]
    // Конструктор для создания экземпляра TextGeneration
    fn new(
        model: Model,         // Модель для генерации текста
        tokenizer: Tokenizer, // Токенизатор для преобразования текста в токены и наоборот
        seed: u64,            // Сид для инициализации генератора случайных чисел
        temp: Option<f64>,    // Температура для управления случайностью в генерации
        top_p: Option<f64>,   // Порог вероятности для семплирования nucleus
        repeat_penalty: f32,  // Штраф за повторение токенов
        repeat_last_n: usize, // Размер контекста для учета при штрафе за повторение
        device: &Device,      // Устройство для выполнения вычислений (CPU или GPU)
    ) -> Self {
        let logits_processor = LogitsProcessor::new(seed, temp, top_p);
        Self {
            model,
            tokenizer: TokenOutputStream::new(tokenizer),
            logits_processor,
            repeat_penalty,
            repeat_last_n,
            device: device.clone(),
        }
    }

    // Метод для запуска генерации текста
    fn run(&mut self, prompt: &str, sample_len: usize) -> Result<()> {
        use std::io::Write;
        self.tokenizer.clear();
        let mut tokens = self
            .tokenizer
            .tokenizer()
            .encode(prompt, true)
            .map_err(E::msg)?
            .get_ids()
            .to_vec();
        for &t in tokens.iter() {
            if let Some(t) = self.tokenizer.next_token(t)? {
                print!("{t}")
            }
        }
        std::io::stdout().flush()?;

        let mut generated_tokens = 0usize;
        let eos_token = match self.tokenizer.get_token("</s>") {
            Some(token) => token,
            None => anyhow::bail!("cannot find the </s> token"),
        };
        let start_gen = std::time::Instant::now();
        for index in 0..sample_len {
            let context_size = if index > 0 { 1 } else { tokens.len() };
            let start_pos = tokens.len().saturating_sub(context_size);
            let ctxt = &tokens[start_pos..];
            let input = Tensor::new(ctxt, &self.device)?.unsqueeze(0)?;

            let logits = self.model.forward(&input, start_pos)?;

            let logits = logits.squeeze(0)?.squeeze(0)?.to_dtype(DType::F32)?;
            let logits = if self.repeat_penalty == 1. {
                logits
            } else {
                let start_at = tokens.len().saturating_sub(self.repeat_last_n);
                candle_transformers::utils::apply_repeat_penalty(
                    &logits,
                    self.repeat_penalty,
                    &tokens[start_at..],
                )?
            };

            let next_token = self.logits_processor.sample(&logits)?;
            tokens.push(next_token);
            generated_tokens += 1;
            if next_token == eos_token {
                break;
            }
            if let Some(t) = self.tokenizer.next_token(next_token)? {
                print!("{t}");
                std::io::stdout().flush()?;
            }
        }
        let dt = start_gen.elapsed();
        if let Some(rest) = self.tokenizer.decode_rest().map_err(E::msg)? {
            print!("{rest}");
        }
        std::io::stdout().flush()?;
        println!(
            "\n{generated_tokens} tokens generated ({:.2} token/s)",
            generated_tokens as f64 / dt.as_secs_f64(),
        );
        Ok(())
    }
}

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Включение трассировки (создает файл trace-timestamp.json).
    #[arg(long)]
    tracing: bool,

    /// Запуск на CPU вместо GPU.
    #[arg(long)]
    cpu: bool,

    #[arg(long)]
    use_flash_attn: bool,

    /// Текстовый запрос для генерации текста.
    #[arg(long)]
    prompt: String,

    /// Температура, используемая для генерации образцов.
    #[arg(long)]
    temperature: Option<f64>,

    /// Порог вероятности семплирования nucleus.
    #[arg(long)]
    top_p: Option<f64>,

    /// Сид для генерации случайных образцов.
    #[arg(long, default_value_t = 299792458)]
    seed: u64,

    /// Длина генерируемого образца (в токенах).
    #[arg(long, default_value_t = 100)]
    sample_len: usize,

    /// Штраф за повторение токенов, 1 означает отсутствие штрафа.
    #[arg(long, default_value_t = 1.1)]
    repeat_penalty: f32,

    /// Размер контекста для учета при штрафе за повторение.
    #[arg(long, default_value_t = 64)]
    repeat_last_n: usize,
}

fn main() -> Result<()> {
    let args = Args::parse();

    let _guard = if args.tracing {
        let (chrome_layer, guard) = ChromeLayerBuilder::new().build();
        tracing_subscriber::registry().with(chrome_layer).init();
        Some(guard)
    } else {
        None
    };

    // Печать информации о поддержке SIMD и других оптимизациях (если это актуально для вашего проекта)
    info!(
        "avx: {}, neon: {}, simd128: {}, f16c: {}",
        candle_core::utils::with_avx(),
        candle_core::utils::with_neon(),
        candle_core::utils::with_simd128(),
        candle_core::utils::with_f16c()
    );

    // Загрузка токенизатора из локального файла
    let tokenizer_filename = "model/tokenizer.json";
    let tokenizer = Tokenizer::from_file(tokenizer_filename).map_err(E::msg)?;

    // Загрузка конфигурации модели из локального файла
    let config_path = "model/config.json";
    let config = Config::from_file(config_path, args.use_flash_attn)?;

    // Определение устройства для запуска модели
    let device = utils::device(args.cpu)?;
    let dtype = if device.is_cuda() {
        DType::F16
    } else {
        DType::F32
    };

    // Загрузка модели из локальных файлов
    let model_files = vec![
        "model/model-00001-of-00002.safetensors",
        "model/model-00002-of-00002.safetensors",
    ];
    let vb = unsafe { VarBuilder::from_mmaped_safetensors(&model_files, dtype, &device)? };

    let model = Model::new(&config, vb)?;

    // Инициализация и запуск генерации текста
    let mut text_generation = TextGeneration::new(
        model,
        tokenizer,
        args.seed,
        args.temperature,
        args.top_p,
        args.repeat_penalty,
        args.repeat_last_n,
        &device,
    );
    text_generation.run(&args.prompt, args.sample_len)?;

    Ok(())
}
