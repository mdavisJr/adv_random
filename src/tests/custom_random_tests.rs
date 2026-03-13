use crate::random_trait::RandomTrait;

#[test]
#[cfg_attr(feature = "rand", ignore)]
#[cfg(not(feature="rand"))]
fn no_default_rand() {
    let test: Box<dyn RandomTrait + Send + Sync> = Box::new(TestNoDefaultRandom{});
    #[cfg(not(feature="rand"))]
    set_random_trait(test);
    let random_result = random_numbers(&Settings::new(&[
        Box::new(NumberRange::all(1, 20))
    ], 10));
    match random_result.numbers() {
        Ok(numbers) => {
            assert!(numbers.len() == 10);
            assert!(numbers.iter().all(|x| *x >= 1 && *x <= 20));
            println!("{:?}", numbers);
        },
        _ => panic!("{}", random_result)
    }
}


#[derive(Debug, Clone)]
pub struct TestNoDefaultRandom {}


// impl TestNoDefaultRandom {
//     pub fn new() -> Box<dyn RandomTrait> {
//         return Box::new(TestNoDefaultRandom {  });
//     }
// }

impl RandomTrait for TestNoDefaultRandom {
    fn get_number(&self, _min: usize, _max: usize) -> usize {
        //static ARRAY: OnceLock<Mutex<Vec<u8>>> = OnceLock::new();
        //ARRAY.get_or_init(|| Mutex::new(vec![1,2,3,4,5,6,7,8,9,10,11,12,13,14,15,16,17,18,19,20]));
        //return ARRAY.get().unwrap().lock().unwrap().pop().unwrap() as usize;
        return 1;
    }

    fn get_bool(&self) -> bool {
        //static ARRAY: OnceLock<Mutex<Vec<bool>>> = OnceLock::new();
        //ARRAY.get_or_init(|| Mutex::new(vec![true,false,true,false,true,false,true,false,true,false,true,false]));
        //return ARRAY.get().unwrap().lock().unwrap().pop().unwrap();
        return true;
    }
}