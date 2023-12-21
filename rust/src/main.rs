// mod key_gen_face_five;
// mod key_gen_face_five_parallel;

mod key_gen_face_five_v2;
// mod key_gen_face_seven_v2;

mod key_gen_face_five_v2_parallel;
mod key_gen_face_five_v2_parallel_steffahn;

mod key_gen_face_seven_v2_parallel;
mod key_gen_face_seven_v2_parallel_steffahn;

mod tester;

fn main() {
    // key_gen_face_five::main();
    // key_gen_face_five_parallel::main();

    key_gen_face_five_v2::main();
    // key_gen_face_seven_v2::main();

    key_gen_face_five_v2_parallel::main();
    key_gen_face_five_v2_parallel_steffahn::main();

    key_gen_face_seven_v2_parallel::main();
    key_gen_face_seven_v2_parallel_steffahn::main();

    // tester::main();
}
