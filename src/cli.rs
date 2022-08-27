pub fn setup() -> clap::ArgMatches {
    let create: clap::Arg = clap::Arg::with_name("create")
        .short('c')
        .long("create")
        .takes_value(true)
        .value_name("TEXT")
        .help("Create new todo item")
        .required(false);


    let read = clap::Arg::with_name("read")
        .short('r')
        .long("read")
        .takes_value(true)
        .value_name("ID")
        .help("Read todo item details")
        .required(false);


    let update = clap::Arg::with_name("update")
        .short('u')
        .long("update")
        .takes_value(true)
        .multiple(true)
        .value_name("TEXT")
        .help("Update todo item with new text")
        .required(false);


    let delete = clap::Arg::with_name("delete")
        .short('d')
        .long("delete")
        .takes_value(true)
        .value_name("ID")
        .help("Delete todo item")
        .required(false);


    let list = clap::Arg::with_name("list")
        .short('l')
        .long("list")
        .help("list all todo item")
        .required(false);


    let app = clap::Command::new("doit")
        .version("0.0.1")
        .about("Portable CLI Task Manager in Rust")
        .author("Nariman Huseynov <github.com/hnariman>")
        .arg(create)
        .arg(read)
        .arg(update)
        .arg(delete)
        .arg(list)
        .get_matches();
    app
}
