pub fn find_by_id_mut<T, F>(items: &mut [T], id: u32, id_getter: F) -> Option<&mut T>
where
    F: Fn(&T) -> u32,
{
    for item in items.iter_mut() {
            if id_getter(item) == id {
                return Some(item);
            }
        }
        None
}