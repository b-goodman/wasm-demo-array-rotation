use wasm_bindgen::prelude::*;

// const rotLeft2D = <T>(a: Array<Array<T>>) => {
// 	// mxn -> nxm array
// 	const m = a.length; //rows
// 	const n = a[0].length; //columns
// 	const newArray = new Array<Array<T>>(n);

// 	for (let i = 0; i < n; i++) {
// 		let newRow = new Array<T>(m);
// 		for (let k = 0; k < m; k++ ) {
// 			newRow[k] = a[k][n-i-1]
// 		}
// 		newArray[i] = newRow;
// 	};
// 	return newArray;
// }

pub type Vec2<T> = Vec<Vec<T>>;

#[wasm_bindgen]
pub fn rotate_left_2d(vec_input: &JsValue) -> JsValue {
    let vec_input_ser: Vec2<f32> = vec_input.into_serde().expect("Error de-serializing");
    // takes a MxN array and returns a NxM array
    let m = vec_input_ser.len();
    let n = vec_input_ser[0].len();
    let mut new_vec = Vec::new();

    for i in 0..n {
        let mut new_row = Vec::new();
        for k in 0..m {
            // the assignment of the element from `vec_input_ser` to `rotated` means that
            // the elem. would no longer exist in the vector.  This is prohibited though since
            // values cannot be moved out of indexed content.  Therefore, the element is actually copied
            // during its assignment.  Thus its type must implement the 'Copy' attribute.
            // An alternative is to borrow the element, i.e., 
            // let rotated = &vec_input_ser[k][n - i - 1];
            // and would give `rotated` as type &f32.
            let rotated = vec_input_ser[k][n - i - 1];
            new_row.push(rotated);
        }
        new_vec.push(new_row);
    }
    return JsValue::from_serde(&new_vec).unwrap();
}

/// Right rotation is exactly the same as left except that the
/// element to be rotated is indexed by [m - 1 - k][i]
/// instead of [k][n - i - 1] as in the case of left.
pub fn rotate_right_2d<T>(vec_input: Vec2<T>) -> Vec2<T>
where
    // we either require that type T implements the 'Copy' trait
    // or have our inner map collect type &T from `vec_input`.  This
    // would change our return type to Vec2<&T> - which probably
    // wouldn't be an issue as it wont be directly exported by wasm
    // and exporting borrowed types is prohibited.
    T: Copy,
{
    let m = vec_input.len();
    let n = vec_input[0].len();
    return (0..n)
        .map(|i| (0..m).map(|k| vec_input[m - 1 - k][i]).collect())
        .collect();
}
