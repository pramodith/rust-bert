// Copyright 2019-present, the HuggingFace Inc. team, The Google AI Language Team and Facebook, Inc.
// Copyright 2019 Guillaume Becquin
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//     http://www.apache.org/licenses/LICENSE-2.0
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

extern crate failure;
extern crate dirs;

use std::path::PathBuf;
use tch::Device;
use rust_bert::pipelines::generation::{LanguageGenerator, GPT2Generator};


fn main() -> failure::Fallible<()> {
    //    Resources paths
    let mut home: PathBuf = dirs::home_dir().unwrap();
    home.push("rustbert");
    home.push("gpt2");
//    home.push("openai-gpt");
    let config_path = &home.as_path().join("config.json");
    let vocab_path = &home.as_path().join("vocab.txt");
    let merges_path = &home.as_path().join("merges.txt");
    let weights_path = &home.as_path().join("model.ot");

//    Set-up masked LM model
    let device = Device::Cpu;

//    let model = OpenAIGenerator::new(vocab_path, merges_path, config_path, weights_path, device)?;
    let model = GPT2Generator::new(vocab_path, merges_path, config_path, weights_path, device)?;

    let input_context = "Dog Dog Dog The The The The Dog Dog";
    let _output = model.generate(Some(input_context), 0, 40, true, false, 1, 1.0,
                                0, 0.9, 1.1, 1.0, 3, 1, None);
//    println!("{:?}", output);
//    output.print();
    Ok(())
}