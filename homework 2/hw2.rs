pub fn log( n: i32, base :i32 ) -> i32
{
    if base <= 1
    {
        return 0;
    }
    let mut i: i32 = n;
    let mut counter: i32 = 0;
    while i >= 1
    {
        i = i / base;
	counter += 1;
    }
    return counter-1;
}

pub fn double_list( list: &mut Vec<f32> )
{
    for i in list.iter_mut()
    {
        *i = i.powf(2.0) as f32;
    }
}

pub fn sum_list( list : & Vec<f32> ) -> f32
{ 
    let mut result: f32 = 0.0;
    for i in list.iter()
    {
        result += i;
    }
    return result;
}

pub fn average_list( list : & Vec<f32> ) -> f32
{
    return sum_list(&list) / list.len() as f32;
}

pub fn std_dev( list: Vec<f32> ) -> f32
{
    let mut result: f32 = 0.0;
    let mean: f32 = average_list(&list);
    for i in list.iter()
    {
        result += (i - mean).powf(2.0);
    }
    return (result / list.len() as f32).sqrt();

}
