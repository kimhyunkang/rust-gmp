#[link(name = "gmp",
       vers = "0.1.0",
       uuid = "a92b7244-82a4-4825-9951-1d475b1296f9",
       url = "https://github.com/thestinger/rust-gmp")];

#[comment = "gmp bindings"];
#[license = "MIT"];
#[crate_type = "lib"];

extern mod std;

use core::clone::Clone;
use core::from_str::FromStr;
use core::libc::{c_char, c_double, c_int, c_long, c_ulong, c_void, size_t};
use core::num::{IntConvertible, One, Zero};
use core::num::IntConvertible::from_int;
use core::ptr::{addr_of, to_mut_unsafe_ptr};
use core::str::as_c_str;
use core::vec;

#[abi = "rust-intrinsic"]
extern "rust-intrinsic" mod rusti {
  fn init<T>() -> T;
}

struct mpz_struct {
    _mp_alloc: c_int,
    _mp_size: c_int,
    _mp_d: *c_void
}

struct mpq_struct {
    _mp_num: mpz_struct,
    _mp_den: mpz_struct
}

type mp_exp_t = c_long;

struct mpf_struct {
    _mp_prec: c_int,
    _mp_size: c_int,
    _mp_exp: mp_exp_t,
    _mp_d: *c_void
}

struct gmp_randstate_struct {
    _mp_seed: mpz_struct,
    _mp_alg: c_int,
    _mp_algdata: *c_void
}

type mp_bitcnt_t = c_ulong;
type mpz_srcptr = *const mpz_struct;
type mpz_ptr = *mut mpz_struct;
type mpq_srcptr = *const mpq_struct;
type mpq_ptr = *mut mpq_struct;
type mpf_srcptr = *const mpf_struct;
type mpf_ptr = *mut mpf_struct;
type gmp_randstate_t = *mut gmp_randstate_struct;

#[link_args = "-lgmp"]
extern "C" {
    fn __gmpz_init(x: mpz_ptr);
    fn __gmpz_init2(x: mpz_ptr, n: mp_bitcnt_t);
    fn __gmpz_init_set(rop: mpz_ptr, op: mpz_srcptr);
    fn __gmpz_init_set_ui(rop: mpz_ptr, op: c_ulong);
    fn __gmpz_init_set_str(rop: mpz_ptr, str: *c_char, base: c_int) -> c_int;
    fn __gmpz_clear(x: mpz_ptr);
    fn __gmpz_realloc2(x: mpz_ptr, n: mp_bitcnt_t);
    fn __gmpz_set(rop: mpz_ptr, op: mpz_srcptr);
    fn __gmpz_set_str(rop: mpz_ptr, str: *c_char, base: c_int) -> c_int;
    fn __gmpz_get_str(str: *c_char, base: c_int, op: mpz_srcptr) -> *c_char;
    fn __gmpz_sizeinbase(op: mpz_srcptr, base: c_int) -> size_t;
    fn __gmpz_cmp(op1: mpz_srcptr, op2: mpz_srcptr) -> c_int;
    fn __gmpz_cmp_ui(op1: mpz_srcptr, op2: c_ulong) -> c_int;
    fn __gmpz_add(rop: mpz_ptr, op1: mpz_srcptr, op2: mpz_srcptr);
    fn __gmpz_sub(rop: mpz_ptr, op1: mpz_srcptr, op2: mpz_srcptr);
    fn __gmpz_mul(rop: mpz_ptr, op1: mpz_srcptr, op2: mpz_srcptr);
    fn __gmpz_mul_2exp(rop: mpz_ptr, op1: mpz_srcptr, op2: mp_bitcnt_t);
    fn __gmpz_neg(rop: mpz_ptr, op: mpz_srcptr);
    fn __gmpz_abs(rop: mpz_ptr, op: mpz_srcptr);
    fn __gmpz_tdiv_q(q: mpz_ptr, n: mpz_srcptr, d: mpz_srcptr);
    fn __gmpz_fdiv_q_2exp(q: mpz_ptr, n: mpz_srcptr, b: mp_bitcnt_t);
    fn __gmpz_mod(r: mpz_ptr, n: mpz_srcptr, d: mpz_srcptr);
    fn __gmpz_and(rop: mpz_ptr, op1: mpz_srcptr, op2: mpz_srcptr);
    fn __gmpz_ior(rop: mpz_ptr, op1: mpz_srcptr, op2: mpz_srcptr);
    fn __gmpz_xor(rop: mpz_ptr, op1: mpz_srcptr, op2: mpz_srcptr);
    fn __gmpz_com(rop: mpz_ptr, op: mpz_srcptr);
    fn __gmpz_popcount(op: mpz_srcptr) -> mp_bitcnt_t;
    fn __gmpz_hamdist(op1: mpz_srcptr, op2: mpz_srcptr) -> mp_bitcnt_t;
    fn __gmpz_setbit(rop: mpz_ptr, bit_index: mp_bitcnt_t);
    fn __gmpz_clrbit(rop: mpz_ptr, bit_index: mp_bitcnt_t);
    fn __gmpz_combit(rop: mpz_ptr, bit_index: mp_bitcnt_t);
    fn __gmpz_tstbit(rop: mpz_srcptr, bit_index: mp_bitcnt_t) -> c_int;
    fn __gmpz_gcd(rop: mpz_ptr, op1: mpz_srcptr, op2: mpz_srcptr);
    fn __gmpz_lcm(rop: mpz_ptr, op1: mpz_srcptr, op2: mpz_srcptr);
    fn __gmpz_invert(rop: mpz_ptr, op1: mpz_srcptr, op2: mpz_srcptr) -> c_int;
    fn __gmpz_import(rop: mpz_ptr, count: size_t, order: c_int, size: size_t,
                     endian: c_int, nails: size_t, op: *const c_void);
    fn __gmp_randinit_default(state: gmp_randstate_t);
    fn __gmp_randinit_mt(state: gmp_randstate_t);
    fn __gmp_randinit_lc_2exp(state: gmp_randstate_t, a: mpz_srcptr, c: c_ulong, m2exp: mp_bitcnt_t);
    fn __gmp_randinit_lc_2exp_size(state: gmp_randstate_t, size: mp_bitcnt_t);
    fn __gmp_randinit_set(state: gmp_randstate_t, op: *const gmp_randstate_struct);
    fn __gmp_randclear(state: gmp_randstate_t);
    fn __gmp_randseed(state: gmp_randstate_t, seed: mpz_srcptr);
    fn __gmp_randseed_ui(state: gmp_randstate_t, seed: c_ulong);
    fn __gmpz_urandomb(rop: mpz_ptr, state: gmp_randstate_t, n: mp_bitcnt_t);
    fn __gmpz_urandomm(rop: mpz_ptr, state: gmp_randstate_t, n: mpz_srcptr);
    fn __gmpq_init(x: mpq_ptr);
    fn __gmpq_clear(x: mpq_ptr);
    fn __gmpq_set(rop: mpq_ptr, op: mpq_srcptr);
    fn __gmpq_set_z(rop: mpq_ptr, op: mpz_srcptr);
    fn __gmpq_set_ui(rop: mpq_ptr, op1: c_ulong, op2: c_ulong);
    fn __gmpq_set_d(rop: mpq_ptr, op: c_double);
    fn __gmpq_set_f(rop: mpq_ptr, op: mpf_srcptr);
    fn __gmpq_cmp(op1: mpq_srcptr, op2: mpq_srcptr) -> c_int;
    fn __gmpq_cmp_ui(op1: mpq_srcptr, num2: c_ulong, den2: c_ulong) -> c_int;
    fn __gmpq_equal(op1: mpq_srcptr, op2: mpq_srcptr) -> c_int;
    fn __gmpq_add(sum: mpq_ptr, addend1: mpq_srcptr, addend2: mpq_srcptr);
    fn __gmpq_sub(difference: mpq_ptr, minuend: mpq_srcptr, subtrahend: mpq_srcptr);
    fn __gmpq_mul(product: mpq_ptr, multiplier: mpq_srcptr, multiplicand: mpq_srcptr);
    fn __gmpq_div(product: mpq_ptr, multiplier: mpq_srcptr, multiplicand: mpq_srcptr);
    fn __gmpq_neg(negated_operand: mpq_ptr, operand: mpq_srcptr);
    fn __gmpq_abs(rop: mpq_ptr, op: mpq_srcptr);
    fn __gmpq_inv(inverted_number: mpq_ptr, number: mpq_srcptr);
    fn __gmpq_get_num(numerator: mpz_ptr, rational: mpq_srcptr);
    fn __gmpq_get_den(denominator: mpz_ptr, rational: mpq_srcptr);
    fn __gmpf_init2(x: mpf_ptr, prec: mp_bitcnt_t);
    fn __gmpf_init_set(rop: mpf_ptr, op: mpf_srcptr);
    fn __gmpf_clear(x: mpf_ptr);
    fn __gmpf_get_prec(op: mpf_srcptr) -> mp_bitcnt_t;
    fn __gmpf_set_prec(rop: mpf_srcptr, prec: mp_bitcnt_t);
    fn __gmpf_set(rop: mpf_ptr, op: mpf_srcptr);
    fn __gmpf_cmp(op1: mpf_srcptr, op2: mpf_srcptr) -> c_int;
    fn __gmpf_cmp_d(op1: mpf_srcptr, op2: c_double) -> c_int;
    fn __gmpf_cmp_ui(op1: mpf_srcptr, op2: c_ulong) -> c_int;
    fn __gmpf_reldiff(rop: mpf_ptr, op1: mpf_srcptr, op2: mpf_srcptr);
    fn __gmpf_add(rop: mpf_ptr, op1: mpf_srcptr, op2: mpf_srcptr);
    fn __gmpf_sub(rop: mpf_ptr, op1: mpf_srcptr, op2: mpf_srcptr);
    fn __gmpf_mul(rop: mpf_ptr, op1: mpf_srcptr, op2: mpf_srcptr);
    fn __gmpf_div(rop: mpf_ptr, op1: mpf_srcptr, op2: mpf_srcptr);
    fn __gmpf_neg(rop: mpf_ptr, op: mpf_srcptr);
    fn __gmpf_abs(rop: mpf_ptr, op: mpf_srcptr);
    fn __gmpf_ceil(rop: mpf_ptr, op: mpf_srcptr);
    fn __gmpf_floor(rop: mpf_ptr, op: mpf_srcptr);
    fn __gmpf_trunc(rop: mpf_ptr, op: mpf_srcptr);
}

pub struct Mpz {
    priv mpz: mpz_struct,
}

impl Drop for Mpz {
    // FIXME: #4514, avoid transmute
    fn finalize(&self) { unsafe { __gmpz_clear(cast::transmute(&self.mpz)) } }
}

impl Mpz {
    fn new() -> Mpz {
        unsafe {
            let mut mpz = rusti::init(); // TODO: switch to rusti::uninit when implemented
            __gmpz_init(&mut mpz);
            Mpz { mpz: mpz }
        }
    }

    fn new_reserve(n: c_ulong) -> Mpz {
        unsafe {
            let mut mpz = rusti::init(); // TODO: switch to rusti::uninit when implemented
            __gmpz_init2(&mut mpz, n);
            Mpz { mpz: mpz }
        }
    }

    fn reserve(&mut self, n: c_ulong) {
        if ((self.bit_length() as c_ulong) < n) {
            unsafe { __gmpz_realloc2(&mut self.mpz, n) }
        }
    }

    fn from_str_radix(s: &str, base: uint) -> Option<Mpz> {
        unsafe {
            assert!(base == 0 || (base >= 2 && base <= 62));
            let mut mpz = rusti::init(); // TODO: switch to rusti::uninit when implemented
            let mpz_ptr = to_mut_unsafe_ptr(&mut mpz);
            let r = as_c_str(s, { |s| __gmpz_init_set_str(mpz_ptr, s, base as c_int) });
            if r == 0 {
                Some(Mpz { mpz: mpz })
            } else {
                __gmpz_clear(mpz_ptr);
                None
            }
        }
    }

    fn set(&mut self, other: &Mpz) {
        unsafe { __gmpz_set(&mut self.mpz, addr_of(&other.mpz)) }
    }

    // TODO: too easy to forget to check this return value - rename?
    fn set_from_str_radix(&mut self, s: &str, base: uint) -> bool {
        assert!(base == 0 || (base >= 2 && base <= 62));
        unsafe {
            let mpz = to_mut_unsafe_ptr(&mut self.mpz);
            as_c_str(s, { |s| __gmpz_set_str(mpz, s, base as c_int) }) == 0
        }
    }

    // TODO: fail on an invalid base
    fn to_str_radix(&self, base: int) -> ~str {
        unsafe {
            let len = __gmpz_sizeinbase(addr_of(&self.mpz), base as c_int) as uint + 2;
            let dst = vec::from_elem(len, '0');
            let pdst = vec::raw::to_ptr(dst);

            str::raw::from_c_str(__gmpz_get_str(pdst as *c_char, base as c_int,
                                                addr_of(&self.mpz)))
        }
    }

    fn bit_length(&self) -> uint {
        unsafe { __gmpz_sizeinbase(addr_of(&self.mpz), 2) as uint }
    }

    fn compl(&self) -> Mpz {
        unsafe {
            let mut res = Mpz::new();
            __gmpz_com(&mut res.mpz, addr_of(&self.mpz));
            res
        }
    }

    fn abs(&self) -> Mpz {
        unsafe {
            let mut res = Mpz::new();
            __gmpz_abs(&mut res.mpz, addr_of(&self.mpz));
            res
        }
    }

    fn gcd(&self, other: &Mpz) -> Mpz {
        unsafe {
            let mut res = Mpz::new();
            __gmpz_gcd(&mut res.mpz, addr_of(&self.mpz), addr_of(&other.mpz));
            res
        }
    }

    fn lcm(&self, other: &Mpz) -> Mpz {
        unsafe {
            let mut res = Mpz::new();
            __gmpz_lcm(&mut res.mpz, addr_of(&self.mpz), addr_of(&other.mpz));
            res
        }
    }

    // TODO: handle a zero modulo
    fn invert(&self, modulo: &Mpz) -> Option<Mpz> {
        unsafe {
            let mut res = Mpz::new();
            if __gmpz_invert(&mut res.mpz, addr_of(&self.mpz),
                             addr_of(&modulo.mpz)) == 0 {
                None
            } else {
                Some(res)
            }
        }
    }

    fn popcount(&self) -> uint {
        unsafe { __gmpz_popcount(addr_of(&self.mpz)) as uint }
    }

    fn hamdist(&self, other: &Mpz) -> uint {
        unsafe { __gmpz_hamdist(addr_of(&self.mpz), addr_of(&other.mpz)) as uint }
    }

    fn setbit(&mut self, bit_index: c_ulong) {
        unsafe { __gmpz_setbit(&mut self.mpz, bit_index) }
    }

    fn clrbit(&mut self, bit_index: c_ulong) {
        unsafe { __gmpz_clrbit(&mut self.mpz, bit_index) }
    }

    fn combit(&mut self, bit_index: c_ulong) {
        unsafe { __gmpz_combit(&mut self.mpz, bit_index) }
    }

    fn tstbit(&self, bit_index: c_ulong) -> bool {
        unsafe { __gmpz_tstbit(&self.mpz, bit_index) == 1 }
    }
}

impl Clone for Mpz {
    fn clone(&self) -> Mpz {
        unsafe {
            let mut mpz = rusti::init(); // TODO: switch to rusti::uninit when implemented
            __gmpz_init_set(&mut mpz, addr_of(&self.mpz));
            Mpz { mpz: mpz }
        }
    }
}

impl cmp::Eq for Mpz {
    fn eq(&self, other: &Mpz) -> bool {
        unsafe { __gmpz_cmp(addr_of(&self.mpz), addr_of(&other.mpz)) == 0 }
    }
    fn ne(&self, other: &Mpz) -> bool {
        unsafe { __gmpz_cmp(addr_of(&self.mpz), addr_of(&other.mpz)) != 0 }
    }
}

impl cmp::Ord for Mpz {
    fn lt(&self, other: &Mpz) -> bool {
        unsafe { __gmpz_cmp(addr_of(&self.mpz), addr_of(&other.mpz)) < 0 }
    }
    fn le(&self, other: &Mpz) -> bool {
        unsafe { __gmpz_cmp(addr_of(&self.mpz), addr_of(&other.mpz)) <= 0 }
    }
    fn gt(&self, other: &Mpz) -> bool {
        unsafe { __gmpz_cmp(addr_of(&self.mpz), addr_of(&other.mpz)) > 0 }
    }
    fn ge(&self, other: &Mpz) -> bool {
        unsafe { __gmpz_cmp(addr_of(&self.mpz), addr_of(&other.mpz)) >= 0 }
    }
}

impl Add<Mpz, Mpz> for Mpz {
    fn add(&self, other: &Mpz) -> Mpz {
        unsafe {
            let mut res = Mpz::new();
            __gmpz_add(&mut res.mpz, addr_of(&self.mpz), addr_of(&other.mpz));
            res
        }
    }
}

impl Sub<Mpz, Mpz> for Mpz {
    fn sub(&self, other: &Mpz) -> Mpz {
        unsafe {
            let mut res = Mpz::new();
            __gmpz_sub(&mut res.mpz, addr_of(&self.mpz), addr_of(&other.mpz));
            res
        }
    }
}

impl Mul<Mpz, Mpz> for Mpz {
    fn mul(&self, other: &Mpz) -> Mpz {
        unsafe {
            let mut res = Mpz::new();
            __gmpz_mul(&mut res.mpz, addr_of(&self.mpz), addr_of(&other.mpz));
            res
        }
    }
}

impl Div<Mpz, Mpz> for Mpz {
    fn div(&self, other: &Mpz) -> Mpz {
        unsafe {
            if __gmpz_cmp_ui(addr_of(&self.mpz), 0) == 0 {
                fail!(~"divide by zero")
            }

            let mut res = Mpz::new();
            __gmpz_tdiv_q(&mut res.mpz, addr_of(&self.mpz), addr_of(&other.mpz));
            res
        }
    }
}

impl Modulo<Mpz, Mpz> for Mpz {
    fn modulo(&self, other: &Mpz) -> Mpz {
        unsafe {
            if __gmpz_cmp_ui(addr_of(&self.mpz), 0) == 0 {
                fail!(~"divide by zero")
            }

            let mut res = Mpz::new();
            __gmpz_mod(&mut res.mpz, addr_of(&self.mpz), addr_of(&other.mpz));
            res
        }
    }
}

impl Neg<Mpz> for Mpz {
    fn neg(&self) -> Mpz {
        unsafe {
            let mut res = Mpz::new();
            __gmpz_neg(&mut res.mpz, addr_of(&self.mpz));
            res
        }
    }
}

impl IntConvertible for Mpz {
    fn to_int(&self) -> int {
        fail!(~"not implemented")
    }
    fn from_int(other: int) -> Mpz {
        unsafe {
            let mut res = Mpz::new();
            __gmpz_import(&mut res.mpz, 1, 1, int::bytes as size_t, 0, 0,
                          addr_of(&int::abs(other)) as *c_void);
            if int::is_negative(other) {
                let mpz_ptr = to_mut_unsafe_ptr(&mut res.mpz);
                __gmpz_neg(mpz_ptr, mpz_ptr)
            }
            res
        }
    }
}

impl One for Mpz {
    fn one() -> Mpz {
        unsafe {
            let mut mpz = rusti::init(); // TODO: switch to rusti::uninit when implemented
            __gmpz_init_set_ui(&mut mpz, 1);
            Mpz { mpz: mpz }
        }
    }
}

impl Zero for Mpz {
    fn zero() -> Mpz { Mpz::new() }
}

impl BitAnd<Mpz, Mpz> for Mpz {
    fn bitand(&self, other: &Mpz) -> Mpz {
        unsafe {
            let mut res = Mpz::new();
            __gmpz_and(&mut res.mpz, addr_of(&self.mpz), addr_of(&other.mpz));
            res
        }
    }
}

impl BitOr<Mpz, Mpz> for Mpz {
    fn bitor(&self, other: &Mpz) -> Mpz {
        unsafe {
            let mut res = Mpz::new();
            __gmpz_ior(&mut res.mpz, addr_of(&self.mpz), addr_of(&other.mpz));
            res
        }
    }
}

impl BitXor<Mpz, Mpz> for Mpz {
    fn bitxor(&self, other: &Mpz) -> Mpz {
        unsafe {
            let mut res = Mpz::new();
            __gmpz_xor(&mut res.mpz, addr_of(&self.mpz), addr_of(&other.mpz));
            res
        }
    }
}

impl Shl<c_ulong, Mpz> for Mpz {
    fn shl(&self, other: &c_ulong) -> Mpz {
        unsafe {
            let mut res = Mpz::new();
            __gmpz_mul_2exp(&mut res.mpz, addr_of(&self.mpz), *other);
            res
        }
    }
}

impl Shr<c_ulong, Mpz> for Mpz {
    fn shr(&self, other: &c_ulong) -> Mpz {
        unsafe {
            let mut res = Mpz::new();
            __gmpz_fdiv_q_2exp(&mut res.mpz, addr_of(&self.mpz), *other);
            res
        }
    }
}

impl FromStr for Mpz {
    fn from_str(s: &str) -> Option<Mpz> {
        Mpz::from_str_radix(s, 10)
    }
}

impl to_str::ToStr for Mpz {
    fn to_str(&self) -> ~str {
        self.to_str_radix(10)
    }
}

pub struct RandState {
    priv state: gmp_randstate_struct,
}

impl Drop for RandState {
    fn finalize(&self) {
        // FIXME: #4514, avoid transmute
        unsafe { __gmp_randclear(cast::transmute(&self.state)) }
    }
}

impl RandState {
    fn new() -> RandState {
        unsafe {
            // TODO: switch to rusti::uninit when implemented
            let mut state: gmp_randstate_struct = rusti::init();
            __gmp_randinit_default(&mut state);
            RandState { state: state }
        }
    }

    fn new_mt() -> RandState {
        unsafe {
            // TODO: switch to rusti::uninit when implemented
            let mut state: gmp_randstate_struct = rusti::init();
            __gmp_randinit_mt(&mut state);
            RandState { state: state }
        }
    }

    fn new_lc_2exp(a: Mpz, c: c_ulong, m2exp: c_ulong) -> RandState {
        unsafe {
            // TODO: switch to rusti::uninit when implemented
            let mut state: gmp_randstate_struct = rusti::init();
            __gmp_randinit_lc_2exp(&mut state, addr_of(&a.mpz), c, m2exp);
            RandState { state: state }
        }
    }

    fn new_lc_2exp_size(size: c_ulong) -> RandState {
        unsafe {
            // TODO: switch to rusti::uninit when implemented
            let mut state: gmp_randstate_struct = rusti::init();
            __gmp_randinit_lc_2exp_size(&mut state, size);
            RandState { state: state }
        }
    }

    fn seed(&mut self, seed: Mpz) {
        unsafe { __gmp_randseed(&mut self.state, addr_of(&seed.mpz)) }
    }

    fn seed_ui(&mut self, seed: c_ulong) {
        unsafe { __gmp_randseed_ui(&mut self.state, seed) }
    }

    /// Generate a uniform random integer in the range 0 to n-1, inclusive
    fn urandom(&mut self, n: &Mpz) -> Mpz {
        unsafe {
            let mut res = Mpz::new();
            __gmpz_urandomm(&mut res.mpz, &mut self.state, addr_of(&n.mpz));
            res
        }
    }

    /// Generate a uniformly distributed random integer in the range 0 to 2^n−1, inclusive.
    fn urandom_2exp(&mut self, n: c_ulong) -> Mpz {
        unsafe {
            let mut res = Mpz::new();
            __gmpz_urandomb(&mut res.mpz, &mut self.state, n);
            res
        }
    }
}

impl Clone for RandState {
    fn clone(&self) -> RandState {
        unsafe {
            // TODO: switch to rusti::uninit when implemented
            let mut state: gmp_randstate_struct = rusti::init();
            __gmp_randinit_set(&mut state, addr_of(&self.state));
            RandState { state: state }
        }
    }
}

pub struct Mpq {
    priv mpq: mpq_struct,
}

impl Drop for Mpq {
    // FIXME: #4514, avoid transmute
    fn finalize(&self) { unsafe { __gmpq_clear(cast::transmute(&self.mpq)) } }
}

impl Mpq {
    fn new() -> Mpq {
        unsafe {
            let mut mpq = rusti::init(); // TODO: switch to rusti::uninit when implemented
            __gmpq_init(&mut mpq);
            Mpq { mpq: mpq }
        }
    }

    fn set(&mut self, other: &Mpq) {
        unsafe { __gmpq_set(&mut self.mpq, addr_of(&other.mpq)) }
    }

    fn set_z(&mut self, other: &Mpz) {
        unsafe { __gmpq_set_z(&mut self.mpq, addr_of(&other.mpz)) }
    }

    fn set_d(&mut self, other: f64) {
        unsafe { __gmpq_set_d(&mut self.mpq, other) }
    }

    fn set_f(&mut self, other: &Mpf) {
        unsafe { __gmpq_set_f(&mut self.mpq, addr_of(&other.mpf)) }
    }

    fn get_num(&self) -> Mpz {
        unsafe {
            let mut res = Mpz::new();
            __gmpq_get_num(&mut res.mpz, addr_of(&self.mpq));
            res
        }
    }

    fn get_den(&self) -> Mpz {
        unsafe {
            let mut res = Mpz::new();
            __gmpq_get_den(&mut res.mpz, addr_of(&self.mpq));
            res
        }
    }

    fn abs(&self) -> Mpq {
        unsafe {
            let mut res = Mpq::new();
            __gmpq_abs(&mut res.mpq, addr_of(&self.mpq));
            res
        }
    }

    fn invert(&self) -> Mpq {
        unsafe {
            if __gmpq_cmp_ui(addr_of(&self.mpq), 0, 1) == 0 {
                fail!(~"divide by zero")
            }

            let mut res = Mpq::new();
            __gmpq_inv(&mut res.mpq, addr_of(&self.mpq));
            res
        }
    }
}

impl Clone for Mpq {
    fn clone(&self) -> Mpq {
        unsafe {
            let mut res = Mpq::new();
            res.set(self);
            res
        }
    }
}

impl cmp::Eq for Mpq {
    fn eq(&self, other: &Mpq) -> bool {
        unsafe { __gmpq_equal(addr_of(&self.mpq), addr_of(&other.mpq)) != 0 }
    }
    fn ne(&self, other: &Mpq) -> bool {
        unsafe { __gmpq_equal(addr_of(&self.mpq), addr_of(&other.mpq)) == 0 }
    }
}

impl cmp::Ord for Mpq {
    fn lt(&self, other: &Mpq) -> bool {
        unsafe { __gmpq_cmp(addr_of(&self.mpq), addr_of(&other.mpq)) < 0 }
    }
    fn le(&self, other: &Mpq) -> bool {
        unsafe { __gmpq_cmp(addr_of(&self.mpq), addr_of(&other.mpq)) <= 0 }
    }
    fn gt(&self, other: &Mpq) -> bool {
        unsafe { __gmpq_cmp(addr_of(&self.mpq), addr_of(&other.mpq)) > 0 }
    }
    fn ge(&self, other: &Mpq) -> bool {
        unsafe { __gmpq_cmp(addr_of(&self.mpq), addr_of(&other.mpq)) >= 0 }
    }
}

impl Add<Mpq, Mpq> for Mpq {
    fn add(&self, other: &Mpq) -> Mpq {
        unsafe {
            let mut res = Mpq::new();
            __gmpq_add(&mut res.mpq, addr_of(&self.mpq), addr_of(&other.mpq));
            res
        }
    }
}

impl Sub<Mpq, Mpq> for Mpq {
    fn sub(&self, other: &Mpq) -> Mpq {
        unsafe {
            let mut res = Mpq::new();
            __gmpq_sub(&mut res.mpq, addr_of(&self.mpq), addr_of(&other.mpq));
            res
        }
    }
}

impl Mul<Mpq, Mpq> for Mpq {
    fn mul(&self, other: &Mpq) -> Mpq {
        unsafe {
            let mut res = Mpq::new();
            __gmpq_mul(&mut res.mpq, addr_of(&self.mpq), addr_of(&other.mpq));
            res
        }
    }
}

impl Div<Mpq, Mpq> for Mpq {
    fn div(&self, other: &Mpq) -> Mpq {
        unsafe {
            if __gmpq_cmp_ui(addr_of(&self.mpq), 0, 1) == 0 {
                fail!(~"divide by zero")
            }

            let mut res = Mpq::new();
            __gmpq_div(&mut res.mpq, addr_of(&self.mpq), addr_of(&other.mpq));
            res
        }
    }
}

impl Neg<Mpq> for Mpq {
    fn neg(&self) -> Mpq {
        unsafe {
            let mut res = Mpq::new();
            __gmpq_neg(&mut res.mpq, addr_of(&self.mpq));
            res
        }
    }
}

impl IntConvertible for Mpq {
    fn to_int(&self) -> int {
        fail!(~"not implemented")
    }
    fn from_int(other: int) -> Mpq {
        let mut res = Mpq::new();
        unsafe { res.set_z(&from_int(other)); } // purity workaround
        res
    }
}

impl One for Mpq {
    fn one() -> Mpq {
        let mut res = Mpq::new();
        unsafe { __gmpq_set_ui(&mut res.mpq, 1, 1) } // purity
        res
    }
}

impl Zero for Mpq {
    fn zero() -> Mpq { Mpq::new() }
}

pub struct Mpf {
    priv mpf: mpf_struct,
}

impl Drop for Mpf {
    // FIXME: #4514, avoid transmute
    fn finalize(&self) { unsafe { __gmpf_clear(cast::transmute(&self.mpf)) } }
}

impl Mpf {
    fn new(precision: c_ulong) -> Mpf {
        unsafe {
            let mut mpf = rusti::init(); // TODO: switch to rusti::uninit when implemented
            __gmpf_init2(&mut mpf, precision);
            Mpf { mpf: mpf }
        }
    }

    fn set(&mut self, other: &Mpf) {
        unsafe { __gmpf_set(&mut self.mpf, addr_of(&other.mpf)) }
    }

    fn get_prec(&self) -> c_ulong {
        unsafe { __gmpf_get_prec(addr_of(&self.mpf)) }
    }

    fn set_prec(&mut self, precision: c_ulong) {
        unsafe { __gmpf_set_prec(&mut self.mpf, precision) }
    }

    fn abs(&self) -> Mpf {
        unsafe {
            let mut res = Mpf::new(self.get_prec());
            __gmpf_abs(&mut res.mpf, addr_of(&self.mpf));
            res
        }
    }

    fn ceil(&self) -> Mpf {
        unsafe {
            let mut res = Mpf::new(self.get_prec());
            __gmpf_ceil(&mut res.mpf, addr_of(&self.mpf));
            res
        }
    }

    fn floor(&self) -> Mpf {
        unsafe {
            let mut res = Mpf::new(self.get_prec());
            __gmpf_floor(&mut res.mpf, addr_of(&self.mpf));
            res
        }
    }

    fn trunc(&self) -> Mpf {
        unsafe {
            let mut res = Mpf::new(self.get_prec());
            __gmpf_trunc(&mut res.mpf, addr_of(&self.mpf));
            res
        }
    }

    fn reldiff(&self, other: &Mpf) -> Mpf {
        unsafe {
            let mut res = Mpf::new(uint::max(self.get_prec() as uint,
                                         other.get_prec() as uint) as c_ulong);
            __gmpf_reldiff(&mut res.mpf, addr_of(&self.mpf), addr_of(&other.mpf));
            res
        }
    }
}

impl Clone for Mpf {
    fn clone(&self) -> Mpf {
        unsafe {
            let mut mpf = rusti::init(); // TODO: switch to rusti::uninit when implemented
            __gmpf_init_set(&mut mpf, addr_of(&self.mpf));
            Mpf { mpf: mpf }
        }
    }
}

impl cmp::Eq for Mpf {
    fn eq(&self, other: &Mpf) -> bool {
        unsafe { __gmpf_cmp(addr_of(&self.mpf), addr_of(&other.mpf)) == 0 }
    }
    fn ne(&self, other: &Mpf) -> bool {
        unsafe { __gmpf_cmp(addr_of(&self.mpf), addr_of(&other.mpf)) != 0 }
    }
}

static fuzzy_epsilon: c_double = 1.0e-6;

impl std::cmp::FuzzyEq<Mpf> for Mpf {
    fn fuzzy_eq(&self, other: &Mpf) -> bool {
        let diff = self.reldiff(other);
        unsafe { __gmpf_cmp_d(addr_of(&diff.mpf), fuzzy_epsilon) < 0 }
    }

    fn fuzzy_eq_eps(&self, other: &Mpf, epsilon: &Mpf) -> bool {
        self.reldiff(other) < *epsilon
    }
}

impl cmp::Ord for Mpf {
    fn lt(&self, other: &Mpf) -> bool {
        unsafe { __gmpf_cmp(addr_of(&self.mpf), addr_of(&other.mpf)) < 0 }
    }
    fn le(&self, other: &Mpf) -> bool {
        unsafe { __gmpf_cmp(addr_of(&self.mpf), addr_of(&other.mpf)) <= 0 }
    }
    fn gt(&self, other: &Mpf) -> bool {
        unsafe { __gmpf_cmp(addr_of(&self.mpf), addr_of(&other.mpf)) > 0 }
    }
    fn ge(&self, other: &Mpf) -> bool {
        unsafe { __gmpf_cmp(addr_of(&self.mpf), addr_of(&other.mpf)) >= 0 }
    }
}

impl Add<Mpf, Mpf> for Mpf {
    fn add(&self, other: &Mpf) -> Mpf {
        unsafe {
            let mut res = Mpf::new(uint::max(self.get_prec() as uint,
                                             other.get_prec() as uint) as c_ulong);
            __gmpf_add(&mut res.mpf, addr_of(&self.mpf), addr_of(&other.mpf));
            res
        }
    }
}

impl Sub<Mpf, Mpf> for Mpf {
    fn sub(&self, other: &Mpf) -> Mpf {
        unsafe {
            let mut res = Mpf::new(uint::max(self.get_prec() as uint,
                                             other.get_prec() as uint) as c_ulong);
            __gmpf_sub(&mut res.mpf, addr_of(&self.mpf), addr_of(&other.mpf));
            res
        }
    }
}

impl Mul<Mpf, Mpf> for Mpf {
    fn mul(&self, other: &Mpf) -> Mpf {
        unsafe {
            let mut res = Mpf::new(uint::max(self.get_prec() as uint,
                                             other.get_prec() as uint) as c_ulong);
            __gmpf_mul(&mut res.mpf, addr_of(&self.mpf), addr_of(&other.mpf));
            res
        }
    }
}

impl Div<Mpf, Mpf> for Mpf {
    fn div(&self, other: &Mpf) -> Mpf {
        unsafe {
            if __gmpf_cmp_ui(addr_of(&self.mpf), 0) == 0 {
                fail!(~"divide by zero")
            }

            let mut res = Mpf::new(uint::max(self.get_prec() as uint,
                                             other.get_prec() as uint) as c_ulong);
            __gmpf_div(&mut res.mpf, addr_of(&self.mpf), addr_of(&other.mpf));
            res
        }
    }
}

impl Neg<Mpf> for Mpf {
    fn neg(&self) -> Mpf {
        unsafe {
            let mut res = Mpf::new(self.get_prec());
            __gmpf_neg(&mut res.mpf, addr_of(&self.mpf));
            res
        }
    }
}

#[cfg(test)]
mod test_mpz {
    use super::*;
    use core::num::IntConvertible::from_int;
    use core::from_str::FromStr;
    use core::num::{One, Zero};
    use core::libc::c_ulong;

    #[test]
    fn test_set() {
        let mut x: Mpz = from_int(1000);
        let y: Mpz = from_int(5000);
        assert!(x != y);
        x.set(&y);
        assert!(x == y);
    }

    #[test]
    fn test_set_from_str_radix() {
        let mut x: Mpz = from_int(1000);
        let y: Mpz = from_int(5000);
        assert!(x != y);
        assert!(x.set_from_str_radix("5000", 10));
        assert!(x == y);
        assert!(!x.set_from_str_radix("aaaa", 2));
    }

    #[test]
    #[should_fail]
    fn test_from_str_radix_lower_bound() {
        Mpz::from_str_radix("", 1);
    }

    #[test]
    #[should_fail]
    fn test_from_str_radix_upper_bound() {
        Mpz::from_str_radix("", 63);
    }

    #[test]
    #[should_fail]
    fn test_set_from_str_radix_lower_bound() {
        let mut x = Mpz::new();
        x.set_from_str_radix("", 1);
    }

    #[test]
    #[should_fail]
    fn test_set_from_str_radix_upper_bound() {
        let mut x = Mpz::new();
        x.set_from_str_radix("", 63);
    }

    #[test]
    fn test_eq() {
        let x: Mpz = from_int(4242142195);
        let y: Mpz = from_int(4242142195);
        let z: Mpz = from_int(4242142196);

        assert!(x == y);
        assert!(x != z);
        assert!(y != z);
    }

    #[test]
    fn test_ord() {
        let x: Mpz = FromStr::from_str("40000000000000000000000").unwrap();
        let y: Mpz = FromStr::from_str("45000000000000000000000").unwrap();
        let z: Mpz = FromStr::from_str("50000000000000000000000").unwrap();

        assert!(x < y && x < z && y < z);
        assert!(x <= x && x <= y && x <= z && y <= z);
        assert!(z > y && z > x && y > x);
        assert!(z >= z && z >= y && z >= x && y >= x);
    }

    #[test]
    #[should_fail]
    fn test_div_zero() {
        let x = Mpz::new();
        x / x;
    }

    #[test]
    #[should_fail]
    fn test_modulo_zero() {
        let x = Mpz::new();
        x % x;
    }

    #[test]
    fn test_div_round() {
        let x: Mpz = from_int(2);
        let y: Mpz = from_int(3);
        assert!((x / y).to_str() == (2i / 3).to_str());
        assert!((x / -y).to_str() == (2i / -3).to_str());
    }

    #[test]
    fn test_to_str_radix() {
        let x: Mpz = from_int(255);
        assert!(x.to_str_radix(16) == ~"ff");
    }

    #[test]
    fn test_to_str() {
        let x: Mpz = FromStr::from_str("1234567890").unwrap();
        assert!(x.to_str() == ~"1234567890");
    }

    #[test]
    fn test_invalid_str() {
        assert!(FromStr::from_str::<Mpz>("foobar").is_none());
    }

    #[test]
    fn test_clone() {
        let a = from_int::<Mpz>(100);
        let b = a.clone();
        assert!(b == a);
        assert!(a + b == from_int::<Mpz>(200));
    }

    #[test]
    fn test_from_int() {
        let x: Mpz = from_int(150);
        assert!(x.to_str() == ~"150");
        assert!(x == FromStr::from_str("150").unwrap());
    }

    #[test]
    fn test_abs() {
        let x: Mpz = from_int(1000);
        let y: Mpz = from_int(-1000);
        assert!(-x == y);
        assert!(x == -y);
        assert!(x == y.abs());
        assert!(x.abs() == y.abs());
    }

    #[test]
    fn test_bitand() {
        let a = 0b1001_0111;
        let b = 0b1100_0100;
        assert!(from_int::<Mpz>(a) & from_int::<Mpz>(b) == from_int::<Mpz>(a & b));
    }

    #[test]
    fn test_bitor() {
        let a = 0b1001_0111;
        let b = 0b1100_0100;
        assert!(from_int::<Mpz>(a) | from_int::<Mpz>(b) == from_int::<Mpz>(a | b));
    }

    #[test]
    fn test_bitxor() {
        let a = 0b1001_0111;
        let b = 0b1100_0100;
        assert!(from_int::<Mpz>(a) ^ from_int::<Mpz>(b) == from_int::<Mpz>(a ^ b));
    }

    #[test]
    fn test_shifts() {
        let i = 227;
        let j: Mpz = from_int(i);
        assert!((i << 4).to_str() == (j << 4).to_str());
        assert!((-i << 4).to_str() == (-j << 4).to_str());
        assert!((i >> 4).to_str() == (j >> 4).to_str());
        assert!((-i >> 4).to_str() == (-j >> 4).to_str());
    }

    #[test]
    fn test_compl() {
        assert!(from_int::<Mpz>(13).compl().to_str() == (!13i).to_str());
        assert!(from_int::<Mpz>(-442).compl().to_str() == (!-442i).to_str());
    }

    #[test]
    fn test_popcount() {
        Mpz::from_str_radix("1010010011", 2).unwrap().popcount() == 5;
    }

    #[test]
    fn test_hamdist() {
        assert!(from_int::<Mpz>(0b1011_0001).hamdist(&from_int(0b0010_1011)) == 4);
    }

    #[test]
    fn test_bit_length() {
        assert!(from_int::<Mpz>(0b1011_0000_0001_0000).bit_length() == 16);
        assert!(from_int::<Mpz>(0b101).bit_length() == 3);
    }

    #[test]
    fn test_gcd() {
        assert!(from_int::<Mpz>(0).gcd(&from_int(0)) == from_int(0));
        assert!(from_int::<Mpz>(3).gcd(&from_int(6)) == from_int(3));
        assert!(from_int::<Mpz>(18).gcd(&from_int(24)) == from_int(6));
    }

    #[test]
    fn test_lcm() {
        assert!(from_int::<Mpz>(0).lcm(&from_int(5)) == from_int(0));
        assert!(from_int::<Mpz>(5).lcm(&from_int(0)) == from_int(0));
        assert!(from_int::<Mpz>(3).lcm(&from_int(6)) == from_int(6));
        assert!(from_int::<Mpz>(18).lcm(&from_int(24)) == from_int(72));
    }

    #[test]
    fn test_invert() {
        assert!(from_int::<Mpz>(3).invert(&from_int(11)) == Some(from_int(4)));
        assert!(from_int::<Mpz>(4).invert(&from_int(11)) == Some(from_int(3)));
        assert!(from_int::<Mpz>(2).invert(&from_int(5)) == Some(from_int(3)));
        assert!(from_int::<Mpz>(3).invert(&from_int(5)) == Some(from_int(2)));
        assert!(from_int::<Mpz>(2).invert(&from_int(4)).is_none());
    }

    #[test]
    fn test_one() {
        assert!(One::one::<Mpz>() == from_int(1));
    }

    #[test]
    fn test_bit_fiddling() {
        let mut xs = from_int::<Mpz>(0b1010_1000_0010_0011);
        assert!(xs.bit_length() == 16);
        let mut ys = vec::reversed([true, false, true, false,
                                   true, false, false, false,
                                   false, false, true, false,
                                   false, false, true, true]);
        for uint::range(0, xs.bit_length()) |i| {
            assert!(xs.tstbit(i as c_ulong) == ys[i]);
        }
        xs.setbit(0);
        ys[0] = true;
        xs.setbit(3);
        ys[3] = true;
        xs.clrbit(1);
        ys[1] = false;
        xs.clrbit(5);
        ys[5] = false;
        xs.combit(14);
        ys[14] = !ys[14];
        xs.combit(15);
        ys[15] = !ys[15];
        for uint::range(0, xs.bit_length()) |i| {
            assert!(xs.tstbit(i as c_ulong) == ys[i]);
        }
    }
}

#[cfg(test)]
mod test_rand {
    use super::*;
    use core::num::IntConvertible::from_int;

    #[test]
    fn test_randstate() {
        let mut state = RandState::new();
        state.seed_ui(42);
        for uint::range(1, 1000) |_| {
            for int::range(1, 10) |x| {
                let upper = from_int(x);
                assert!(state.urandom(&upper) < upper);
            }
        }
    }
}

#[cfg(test)]
mod test_mpq {
    use super::*;
    use core::num::One;
    use core::num::IntConvertible::from_int;

    #[test]
    fn test_one() {
        assert!(One::one::<Mpq>() == from_int(1));
    }

    #[test]
    #[should_fail]
    fn test_div_zero() {
        let x = Mpq::new();
        x / x;
    }

    #[test]
    #[should_fail]
    fn test_invert_zero() {
        Mpq::new().invert();
    }
}

#[cfg(test)]
mod test_mpf {
    use super::*;

    #[test]
    #[should_fail]
    fn test_div_zero() {
        let x = Mpf::new(100);
        x / x;
    }
}
