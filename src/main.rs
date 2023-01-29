use actix_web::middleware::Logger;
use actix_web::{get, App, HttpResponse, HttpServer, Responder};
use serde::Serialize;


use exitfailure::ExitFailure;
use std::thread;


use rust_bert::bart::{
    BartConfigResources, BartMergesResources, BartModelResources, BartVocabResources,
};
use rust_bert::pipelines::summarization::{SummarizationConfig, SummarizationModel};
use rust_bert::resources::RemoteResource;
use tch::Device;


#[derive(Serialize)]
pub struct GenericResponse {
    pub status: String,
    pub message: String,
}

#[get("/api/hello")]
async fn health_checker_handler() -> impl Responder {
    const MESSAGE: &str = "Build Simple CRUD API with Rust and Actix Web";
    let do_steps = || -> Result<String, ExitFailure> {
        let config_resource = Box::new(RemoteResource::from_pretrained(
            BartConfigResources::DISTILBART_CNN_6_6,
        ));
        let vocab_resource = Box::new(RemoteResource::from_pretrained(
            BartVocabResources::DISTILBART_CNN_6_6,
        ));
        let merges_resource = Box::new(RemoteResource::from_pretrained(
            BartMergesResources::DISTILBART_CNN_6_6,
        ));
        let model_resource = Box::new(RemoteResource::from_pretrained(
            BartModelResources::DISTILBART_CNN_6_6,
        ));

        let summarization_config = SummarizationConfig {
            model_resource,
            config_resource,
            vocab_resource,
            merges_resource: Some(merges_resource),
            num_beams: 1,
            length_penalty: 1.0,
            min_length: 56,
            max_length: Some(142),
            device: Device::Cpu,
            ..Default::default()
        };


        let summarization_model = SummarizationModel::new(summarization_config)?;
//
        let input = ["n the mid-19th century, the Qing dynasty experienced Western imperialism in the Opium Wars with Britain and France. China was forced to pay compensation, open treaty ports, allow extraterritoriality for foreign nationals, and cede Hong Kong to the British[85] under the 1842 Treaty of Nanking, the first of the Unequal Treaties. The First Sino-Japanese War (1894–1895) resulted in Qing China's loss of influence in the Korean Peninsula, as well as the cession of Taiwan to Japan.[86] The Qing dynasty also began experiencing internal unrest in which tens of millions of people died, especially in the White Lotus Rebellion, the failed Taiping Rebellion that ravaged southern China in the 1850s and 1860s and the Dungan Revolt (1862–1877) in the northwest. The initial success of the Self-Strengthening Movement of the 1860s was frustrated by a series of military defeats in the 1880s and 1890s.[citation needed]

In the 19th century, the great Chinese diaspora began. Losses due to emigration were added to by conflicts and catastrophes such as the Northern Chinese Famine of 1876–1879, in which between 9 and 13 million people died.[87] The Guangxu Emperor drafted a reform plan in 1898 to establish a modern constitutional monarchy, but these plans were thwarted by the Empress Dowager Cixi. The ill-fated anti-foreign Boxer Rebellion of 1899–1901 further weakened the dynasty. Although Cixi sponsored a program of reforms, the Xinhai Revolution of 1911–1912 brought an end to the Qing dynasty and established the Republic of China.[88] Puyi, the last Emperor of China, abdicated in 1912"];

           // Credits: WikiNews, CC BY 2.5 license (https://en.wikinews.org/wiki/Astronomers_find_water_vapour_in_atmosphere_of_exoplanet_K2-18b)
        let _output = summarization_model.summarize(&input);
        let mut mutable_string = String::from(_output.join(" "));

        Ok(mutable_string)
    };

    let result=thread::spawn(move|| {


        match do_steps() {
            Ok(report) => {
                report
            }
            Err(err) => {
                "error".to_string()

                // or write a better logic
            }
        }


    }).join().expect("Thread panicked");

    let response_json = &GenericResponse {
        status: "success".to_string(),
        message: result.to_string(),
    };
    HttpResponse::Ok().json(response_json)
}

#[actix_web::main]
async fn main() -> Result<(), ExitFailure> {
    if std::env::var_os("RUST_LOG").is_none() {
        std::env::set_var("RUST_LOG", "actix_web=info");
    }
    env_logger::init();

    println!("Server started successfully");

    HttpServer::new(move || {
        App::new()
            .service(health_checker_handler)
            .wrap(Logger::default())
    })
        .bind(("127.0.0.1", 8000))?
        .run()
        .await?;
    Ok(())
}

