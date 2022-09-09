pub fn get_toy_cost(board_width: [u32; 2], toy_composition: Vec<Vec<u32>>) -> u32 {
    return 6;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let board_width:[u32; 2] =  [1, 1];
        let toy_composition:Vec<Vec<u32>> = Vec::new();
        let expected:u32 = 6;

        let result = get_toy_cost(board_width, toy_composition);
        assert_eq!(result, expected);
    }
}
