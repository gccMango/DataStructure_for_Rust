fn main() {
    // unsafe {
    //     let mut data = 0;
    //     let ref1 = &mut data;
    //     let ptr2 = ref1 as *mut _ ;

    //     // ORDER SWAPPED!
    //     *ref1 += 1;
    //     *ptr2 += 2;

    //     println!("{}", data);
    // }
    // unsafe {
    //     let mut data = [0; 10];
    //     let ref1_at_0 = &mut data[0];           // Reference to 0th element
    //     let ptr2_at_0 = ref1_at_0 as *mut i32;  // Ptr to 0th element
    //     let ptr3_at_1 = ptr2_at_0.add(1);       // Ptr to 1st element

    //     *ptr3_at_1 += 3;
    //     *ptr2_at_0 += 2;
    //     *ref1_at_0 += 1;

    //     // Should be [3, 3, 0, ...]
    //     println!("{:?}", &data[..]);
    // }

    // unsafe {
    //     let mut data = [0; 10];
    //     let ref1_at_0 = &mut data[0]; // Reference to 0th element
    //     let ptr2_at_0 = ref1_at_0 as *mut i32; // Ptr to 0th element
    //     let ptr3_at_0 = ptr2_at_0; // Ptr to 0th element
    //     let ptr4_at_0 = ptr2_at_0.add(0); // Ptr to 0th element
    //     let ptr5_at_0 = ptr3_at_0.add(1).sub(1); // Ptr to 0th element

    //     // An absolute jumbled hash of ptr usages
    //     *ptr3_at_0 += 3;
    //     *ptr2_at_0 += 2;
    //     *ptr4_at_0 += 4;
    //     *ptr5_at_0 += 5;
    //     *ptr3_at_0 += 3;
    //     *ptr2_at_0 += 2;
    //     *ref1_at_0 += 1;

    //     // Should be [20, 0, 0, ...]
    //     println!("{:?}", &data[..]);
    // }

    // unsafe {
    //     let mut data = [0; 10];

    //     let slice1 = &mut data[..];
    //     let (slice2_at_0, slice3_at_1) = slice1.split_at_mut(1);

    //     let ref4_at_0 = &mut slice2_at_0[0]; // Reference to 0th element
    //     let ref5_at_1 = &mut slice3_at_1[0]; // Reference to 1th element
    //     let ptr6_at_0 = ref4_at_0 as *mut i32; // Ptr to 0th element
    //     let ptr7_at_1 = ref5_at_1 as *mut i32; // Ptr to 1th element

    //     *ptr7_at_1 += 7;
    //     *ptr6_at_0 += 6;
    //     *ref5_at_1 += 5;
    //     *ref4_at_0 += 4;

    //     // Should be [10, 12, 0, ...]
    //     println!("{:?}", &data[..]);
    // }

    // unsafe {
    //     let mut data = [0; 10];

    //     let slice1_all = &mut data[..]; // Slice for the entire array
    //     let ptr2_all = slice1_all.as_mut_ptr(); // Pointer for the entire array

    //     let ptr3_at_0 = ptr2_all; // Pointer to 0th elem (the same)
    //     let ptr4_at_1 = ptr2_all.add(1); // Pointer to 1th elem
    //     let ref5_at_0 = &mut *ptr3_at_0; // Reference to 0th elem
    //     let ref6_at_1 = &mut *ptr4_at_1; // Reference to 1th elem

    //     *ref6_at_1 += 6;
    //     *ref5_at_0 += 5;
    //     *ptr4_at_1 += 4;
    //     *ptr3_at_0 += 3;

    //     // Just for fun, modify all the elements in a loop
    //     // (Could use any of the raw pointers for this, they share a borrow!)
    //     for idx in 0..10 {
    //         *ptr2_all.add(idx) += idx;
    //     }

    //     // Safe version of this same code for fun
    //     for (idx, elem_ref) in slice1_all.iter_mut().enumerate() {
    //         *elem_ref += idx;
    //     }

    //     // Should be [8, 12, 4, 6, 8, 10, 12, 14, 16, 18]
    //     println!("{:?}", &data[..]);
    // }

    // unsafe {
    //     let mut data = 10;
    //     let mref1 = &mut data;
    //     let sref2 = &mref1;
    //     let sref3 = sref2;
    //     let sref4 = &*sref2;

    //     // Random hash of shared reference reads
    //     opaque_read(sref3);
    //     opaque_read(sref2);
    //     opaque_read(sref4);
    //     opaque_read(sref2);
    //     opaque_read(sref3);

    //     *mref1 += 1;

    //     opaque_read(&data);
    // }
    // unsafe {
    //     let mut data = 10;
    //     let mref1 = &mut data;
    //     let ptr2 = mref1 as *mut i32;
    //     // let sref3 = &mref1;
    //     // let ptr4 = sref3 as *mut i32;
    //     let sref3 = &*mref1;
    //     let ptr4 = sref3 as *const i32 as *mut i32;

    //     *ptr4 += 4;
    //     opaque_read(sref3);
    //     *ptr2 += 2;
    //     *mref1 += 1;
    
    //     opaque_read(&data);
    // }

    unsafe {
        let mut data = 10;
        let mref1 = &mut data;
        let ptr2 = mref1 as *mut i32;
        let sref3 = &*mref1;
        let ptr4 = sref3 as *const i32 as *mut i32;
    
        opaque_read(&*ptr4);
        opaque_read(sref3);
        *ptr2 += 2;
        *mref1 += 1;
    
        opaque_read(&data);
    }
}

fn opaque_read(val: &i32) {
    println!("{}", val);
}
