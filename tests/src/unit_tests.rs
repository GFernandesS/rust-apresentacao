#[cfg(test)]
mod tests {
    use crate::sum;

    #[test]
    fn when_call_sum_two_numbers_should_return_result_of_adding_two_numbers(){
        let result = sum::sum_two_numbers(1, 2);
        assert_eq!(result, 5);
    }
}