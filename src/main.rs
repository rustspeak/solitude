fn tri_boules<T : Ord>(tableau : &mut [T]){
    let mut fini = true;
    for   i  in 0..tableau.len() -1 {
        if   fini {
            break;
        }
        fini =   true ;

        for  j in  0..tableau.len() - i -1  {
            if   tableau[j]  > tableau[j +  1]  {
                 tableau.swap(j,   j+1);
                 fini = false ;
            }
        }
    }

}
#[cfg(test)]
mod tests {
    use   super::tri_boules;
    #[test]
    fn  cas_simple(){
        let  mut tablau = [23,3,14,56,4];
        tri_boules(&mut tablau);
        assert_eq!(tablau, [3,4,14,23,56]);
    }
}

fn main(){

}
