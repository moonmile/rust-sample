#[cfg(test)]
mod tests {
    use super::* ;
    #[test]
    fn it_works() {

        assert_eq!(2 + 2, 4);
    }

    #[test]
    fn test_add() {
        assert_eq!( 3, add(1,2));
    }

    #[test]
    fn test_add_zero() {
        assert_eq!( 0, add(0,0));
    }
    
    #[test]
    fn test_add_under_zero() {
        assert_eq!( 0, add(-1,-1));
    }

    #[test]
    fn test_add_double() {
        assert_eq!( 1.2, add_double(1.0, 0.2));
        assert_eq!( 0.0, add_double(0.0, 0.0));
        assert_eq!( 0.0, add_double(-1.0, -1.0));
    }

    #[test]
    fn test_add_str() {
        assert_eq!( "masuda tomoaki", add_str("masuda", "tomoaki"));
        assert_eq!( "masuda", add_str("masuda", ""));
        assert_eq!( "", add_str("", ""));
    }
}

fn add(x: i32, y: i32 ) -> i32 {
    // x + y 
    let ans = x + y ;
    if ans < 0 {
        0
    } else {
        ans 
    }
}

fn add_double(x: f32, y: f32 ) -> f32 {
    let ans = x + y;
    if ans < 0.0 {
        0.0
    } else {
        ans 
    }
}

fn add_str( x: &str, y: &str) -> String {
    let ans = format!("{} {}", x, y );
    ans.trim().to_string()
}
