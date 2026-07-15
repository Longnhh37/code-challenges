pub fn map<T, U, F>(input: Vec<T>, mut function: F) -> Vec<U>
where
    F: FnMut(T) -> U,
{
    let mut output = Vec::new();

    for element in input {
        output.push(function(element));
    }

    output
}
