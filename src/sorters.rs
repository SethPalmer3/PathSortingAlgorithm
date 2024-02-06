use crate::graph::Graph;

pub fn merge_sort<I, G>(array: &mut [I], g: &G)
where
    I: Clone + Copy,
    G: Graph<I = I>,{
    if array.len() <= 1{
        return;
    }
    let r = array.len() / 2;
    let mut left = vec![array[0]; r];
    left.copy_from_slice(&array[0..r]);
    let mut right = vec![array[0]; array.len()-r];
    right.clone_from_slice(&array[r..]);
    merge_sort(&mut left, g);
    merge_sort(&mut right, g);
    let mut i = 0;
    let mut j = 0;
    let mut k = 0;
    while i < left.len() && j < right.len(){
        array[k] = left[i];
        let d_l = g.path_dist(&array[..k+1]);
        array[k] = right[j];
        let d_r = g.path_dist(&array[..k+1]);
        if d_r < d_l{
            j+=1;
        }else{
            array[k] = left[i];
            i+=1;
        }
        k+=1;
    }

    while i < left.len(){
        array[k] = left[i];
        i+=1;
        k+=1;
    }
    while j < right.len(){
        array[k] = right[j];
        j+=1;
        k+=1;
    }
}

