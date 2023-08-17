use structopt::StructOpt;

#[derive(Debug, StructOpt)]
#[structopt(
    name = "simple static server",
    about = "plain and simple static file server"
)]
struct Opt {
    /// Activate debug mode
    // short and long flags (-d, --debug) will be deduced from the field's name
    #[structopt(short, long)]
    debug: bool,

    /// Root directory of files
    #[structopt(short = "d", long = "directory", default_value = ".")]
    dir: String,

    /// 404 page, for when a page is not found
    #[structopt(short = "m", long = "missing-page", default_value = "")]
    dir: Option<String>,
}
