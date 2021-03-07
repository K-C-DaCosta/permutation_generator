use clap::{App, Arg};

use permutation_generator;

fn main() {
    let matches = App::new("Permutation generator")
        .version("1.0")
        .author("k.c dacosta")
        .about("prints lists of permutations to stdout")
        .arg(
            Arg::with_name("item_list")
                .short("l")
                .long("list")
                .help("specify the list of items you want listed")
                .takes_value(true)
                .required(true),
        )
        .arg(
            Arg::with_name("delimiter")
                .short("d")
                .long("delim")
                .help("specify the list delimiter(by default its ' ')")
                .takes_value(true),
        )
        .arg(
            Arg::with_name("size")
                .short("s")
                .long("gen_size")
                .takes_value(true)
                .help(
                    "specifies the size of the generated list. By default size is the list length",
                ),
        )
        .get_matches();

    let delim = matches.value_of("delimiter").unwrap_or(" ");
    let list = matches
        .value_of("item_list")
        .expect("list required")
        .split(delim)
        .map(|a| a.into())
        .collect::<Vec<String>>();

    let length = matches
        .value_of("size")
        .map(|a| a.parse::<usize>().ok())
        .flatten()
        .unwrap_or(list.len());

    permutation_generator::print_permutations(&list, length);
}
