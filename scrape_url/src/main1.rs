mod quick_start;
use crate::quick_start::download_to_md::download_file_to_markdown;
use crate::quick_start::func_test::*;

fn main() {
    // test download file to markdown
    download_file_to_markdown();

    // test function
    test_func();
}
