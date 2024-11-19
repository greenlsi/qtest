    // //CORS es esencial para controlar y proteger los accesos entre el frontend y el backend.
    // //Configurarlo en Warp permite que un frontend (React) en un dominio acceda al backend
    // //en otro, manteniendo una estructura segura y funcional para aplicaciones modernas.
    // let cors = warp::cors()
    //     .allow_any_origin()
    //     .allow_methods(vec!["GET", "POST"]);

    // let routes = message_route().or(pulsar_boton(parser)).with(cors); //

    // //Inicia el servidor de Warp con las rutas especificadas y lo configura para
    // // escuchar en 127.0.0.1 en el puerto 3030. El servidor comienza a aceptar
    // // solicitudes en esa dirección y se mantendrá activo hasta que se detenga
    // // manualmente o se cierre el programa.
    // warp::serve(routes).run(([127, 0, 0, 1], 3030)).await;


//use warp::{reject::Reject, Filter};
//Warp organiza rutas y filtros de manera modular y asíncrona, lo que
//permite construir servidores HTTP eficientes en Rust.