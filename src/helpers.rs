pub mod slices {

    pub struct InvalidSliceError(pub String);
    
    pub fn concat_2<T: Clone>(a: &[T], b: &[T]) -> Vec<T> {
	let mut v = Vec::with_capacity(a.len() + b.len());
	v.extend_from_slice(a);
	v.extend_from_slice(b);
	v
    }

    pub fn concat_3<T: Clone>(a: &[T], b: &[T], c: &[T]) -> Vec<T> {
	let mut v = Vec::with_capacity(a.len() + b.len() + c.len());
	v.extend_from_slice(a);
	v.extend_from_slice(b);
	v.extend_from_slice(c);
	v
    }

    // This function has similar behaviors of Java's System.arraycopy, only with slices &[T].
    pub fn copy<T: Clone>(src: &[T], src_pos: usize,
		      dest: &[T], dest_pos: usize,
			  length: usize) -> Result<Vec<T>, InvalidSliceError> {

	if src_pos > src.len()-1 {
	    return Err(InvalidSliceError("src_pos exceeds src length".to_string()))
	}
	if dest_pos > dest.len()-1 {
	    return Err(InvalidSliceError("dest_pos exceeds dest length".to_string()))
	}
	if length + src_pos > src.len() {
	    return Err(InvalidSliceError("Length of dest slice segment exceeds src length.".to_string()))
	}
	if dest_pos + length > dest.len() {
	    return Err(InvalidSliceError("Length of dest slice segment exceeds dest length.".to_string()))
	}
	
	let dest_front = &dest[..dest_pos];
	let dest_back = &dest[(dest_pos+length)..];
	let src_segment = &src[src_pos..(src_pos+length)];
	let v = concat_3(dest_front, src_segment, dest_back);
	Ok(v)
    }
}

#[cfg(test)]
pub mod tests {
    use super::slices::*;

    pub struct TestTable<'a> {
	src: &'a [u8],
	dest: &'a [u8],
	src_pos: usize,
	dest_pos: usize,
	length: usize,
	expected: Result<Vec<u8>, InvalidSliceError>,
    }
    
    #[test]
    pub fn test_concat_2() {
	
	let a = &[0; 16][..];
	let b = &[1; 16][..];
	
	let v = concat_2(&a, &b);
	
	assert_eq!(v.len(), a.len() + b.len());
	assert_eq!(&v[..a.len()], &[0; 16]);
	assert_eq!(&v[a.len()..], &[1; 16]);
    }

    #[test]
    pub fn test_concat_3() {
	let a = &[0; 16][..];
	let b = &[1; 65][..];
	let c = &[2; 8][..];

	let v = concat_3(&a, &b, &c);

	assert_eq!(v.len(), a.len() + b.len() + c.len());
	assert_eq!(&v[..a.len()], a);
	assert_eq!(&v[a.len()..(b.len() + a.len())], b);
	assert_eq!(&v[(b.len() + a.len())..], c);
    }

    #[test]
    pub fn test_copy() {

	let tts = [
	    
	    TestTable {
		src: &[0; 16][..],
		dest: &[1; 16][..],
		src_pos: 5,
		dest_pos: 8,
		length: 6,
		expected: Ok(vec![1, 1, 1, 1, 1, 1, 1, 1, 0, 0, 0, 0, 0, 0, 1, 1]),
	    },

	    TestTable {
		src: &[0; 20][..],
		dest: &[1; 10][..],
		src_pos: 10,
		dest_pos: 2,
		length: 5,
		expected: Ok(vec![1, 1, 0, 0, 0, 0, 0, 1, 1, 1]),
	    },

	    TestTable {
		src: &[0; 50][..],
		dest: &[1; 20][..],
		src_pos: 40,
		dest_pos: 10,
		length: 11,
		expected: Err(InvalidSliceError("Length of dest slice segment exceeds src length.".to_string())),
	    },

	    TestTable {
		src: &[0; 100][..],
		dest: &[1; 50][..],
		src_pos: 10,
		length: 70,
		dest_pos: 1,
		expected: Err(InvalidSliceError("Length of dest slice segment exceeds dest length.".to_string())),
	    },

	    TestTable {
		src: &[0; 5][..],
		dest: &[1; 40][..],
		src_pos: 10,
		dest_pos: 56,
		length: 2,
		expected: Err(InvalidSliceError("src_pos exceeds src length".to_string())),
	    },

	    TestTable {
		src: &[0; 5][..],
		dest: &[1; 40][..],
		src_pos: 3,
		dest_pos: 56,
		length: 2,
		expected: Err(InvalidSliceError("dest_pos exceeds dest length".to_string())),
	    },
	];

	for tt in tts.iter() {
	    let result = copy(&tt.src, tt.src_pos, &tt.dest, tt.dest_pos, tt.length);
	    match &tt.expected {
		Ok(ex) => {
		    match result {
			Ok(v) => {
			    assert_eq!(v.len(), ex.len());
			    assert_eq!(v, *ex);
			},
			Err(_) => panic!("!!"),
		    }
		},
		Err(e) => {
		    let InvalidSliceError(error_msg) = &*e;
		    if tt.src_pos > tt.src.len()-1 {
			assert_eq!(*error_msg, "src_pos exceeds src length".to_string());
		    } else if tt.dest_pos > tt.dest.len()-1 {
			assert_eq!(*error_msg, "dest_pos exceeds dest length".to_string());
		    } else if tt.length + tt.src_pos > tt.src.len() {
			assert_eq!(*error_msg, "Length of dest slice segment exceeds src length.".to_string());
		    } else if tt.dest_pos + tt.length > tt.dest.len() {
			assert_eq!(*error_msg, "Length of dest slice segment exceeds dest length.".to_string());
		    } else {
			panic!("Case not covered");
		    }
		},
	    }
	}
    }
}

    
    
