mod int_list;
mod s_list;

fn main() {
    let mut slist = s_list::SList::new(None);
    for i in 0..10_000_000 {
        slist.add_first(i);
    }
}
