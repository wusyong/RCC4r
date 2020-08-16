#![allow(non_camel_case_types, non_snake_case, non_upper_case_globals, unused_assignments)]
#![feature(ptr_wrapping_offset_from)]

use libc::*;

// print executed instructions
// tokens and classes (operators last and in precedence order)
pub const Brak: c_uint = 164;
pub const Dec: c_uint = 163;
pub const Inc: c_uint = 162;
pub const Mod: c_uint = 161;
pub const Div: c_uint = 160;
pub const Mul: c_uint = 159;
pub const Sub: c_uint = 158;
pub const Add: c_uint = 157;
pub const Shr: c_uint = 156;
pub const Shl: c_uint = 155;
pub const Ge: c_uint = 154;
pub const Le: c_uint = 153;
pub const Gt: c_uint = 152;
pub const Lt: c_uint = 151;
pub const Ne: c_uint = 150;
pub const Eq: c_uint = 149;
pub const And: c_uint = 148;
pub const Xor: c_uint = 147;
pub const Or: c_uint = 146;
pub const Lan: c_uint = 145;
pub const Lor: c_uint = 144;
pub const Cond: c_uint = 143;
pub const Assign: c_uint = 142;
pub const While: c_uint = 141;
pub const Sizeof: c_uint = 140;
pub const Return: c_uint = 139;
pub const Int: c_uint = 138;
pub const If: c_uint = 137;
pub const Enum: c_uint = 136;
pub const Else: c_uint = 135;
pub const Char: c_uint = 134;
pub const Id: c_uint = 133;
pub const Loc: c_uint = 132;
pub const Glo: c_uint = 131;
pub const Sys: c_uint = 130;
pub const Fun: c_uint = 129;
pub const Num: c_uint = 128;
// opcodes
pub const EXIT: c_uint = 38;
pub const MCMP: c_uint = 37;
pub const MSET: c_uint = 36;
pub const FREE: c_uint = 35;
pub const MALC: c_uint = 34;
pub const PRTF: c_uint = 33;
pub const CLOS: c_uint = 32;
pub const READ: c_uint = 31;
pub const OPEN: c_uint = 30;
pub const MOD: c_uint = 29;
pub const DIV: c_uint = 28;
pub const MUL: c_uint = 27;
pub const SUB: c_uint = 26;
pub const ADD: c_uint = 25;
pub const SHR: c_uint = 24;
pub const SHL: c_uint = 23;
pub const GE: c_uint = 22;
pub const LE: c_uint = 21;
pub const GT: c_uint = 20;
pub const LT: c_uint = 19;
pub const NE: c_uint = 18;
pub const EQ: c_uint = 17;
pub const AND: c_uint = 16;
pub const XOR: c_uint = 15;
pub const OR: c_uint = 14;
pub const PSH: c_uint = 13;
pub const SC: c_uint = 12;
pub const SI: c_uint = 11;
pub const LC: c_uint = 10;
pub const LI: c_uint = 9;
pub const LEV: c_uint = 8;
pub const ADJ: c_uint = 7;
pub const ENT: c_uint = 6;
pub const BNZ: c_uint = 5;
pub const BZ: c_uint = 4;
pub const JSR: c_uint = 3;
pub const JMP: c_uint = 2;
pub const IMM: c_uint = 1;
pub const LEA: c_uint = 0;
// types
pub const PTR: c_uint = 2;
pub const INT: c_uint = 1;
pub const CHAR: c_uint = 0;
// identifier offsets (since we can't create an ident struct)
pub const Idsz: c_uint = 9;
pub const HVal: c_uint = 8;
pub const HType: c_uint = 7;
pub const HClass: c_uint = 6;
pub const Val: c_uint = 5;
pub const Type: c_uint = 4;
pub const Class: c_uint = 3;
pub const Name: c_uint = 2;
pub const Hash: c_uint = 1;
pub const Tk: c_uint = 0;

pub static mut p: *mut c_char = 0 as *const c_char as *mut c_char;
pub static mut lp: *mut c_char = 0 as *const c_char as *mut c_char;
// current position in source code
pub static mut data: *mut c_char = 0 as *const c_char as *mut c_char;
// data/bss pointer
pub static mut e: *mut ssize_t = 0 as *const ssize_t as *mut ssize_t;
pub static mut le: *mut ssize_t = 0 as *const ssize_t as *mut ssize_t;
// current position in emitted code
pub static mut id: *mut ssize_t = 0 as *const ssize_t as *mut ssize_t;
// currently parsed identifier
pub static mut sym: *mut ssize_t = 0 as *const ssize_t as *mut ssize_t;
// symbol table (simple list of identifiers)
pub static mut tk: ssize_t = 0;
// current token
pub static mut ival: ssize_t = 0;
// current token value
pub static mut ty: ssize_t = 0;
// current expression type
pub static mut loc: ssize_t = 0;
// local variable offset
pub static mut line: ssize_t = 0;
// current line number
pub static mut src: ssize_t = 0;
// print source and assembly flag
pub static mut debug: ssize_t = 0;

pub unsafe fn next() {
    let mut pp: *mut c_char = 0 as *mut c_char;
    loop {
        tk = *p as ssize_t;
        if !(tk != 0) {
            break;
        }
        p = p.offset(1);
        if tk == '\n' as ssize_t {
            if src != 0 {
                printf(b"%d: %.*s\x00" as *const u8 as *const c_char, line, p.wrapping_offset_from(lp) as ssize_t, lp);
                lp = p;
                while le < e {
                    le = le.offset(1);
                    printf(
                        b"%8.4s\x00" as *const u8 as *const c_char,
                        &*(b"LEA ,IMM ,JMP ,JSR ,BZ  ,BNZ ,ENT ,ADJ ,LEV ,LI  ,LC  ,SI  ,SC  ,PSH ,OR  ,XOR ,AND ,EQ  ,NE  ,LT  ,GT  ,LE  ,GE  ,SHL ,SHR ,ADD ,SUB ,MUL ,DIV ,MOD ,OPEN,READ,CLOS,PRTF,MALC,FREE,MSET,MCMP,EXIT,\x00" as *const u8 as *const c_char).offset((*le * 5 as c_int as ssize_t) as isize) as *const c_char,
                    );
                    if *le <= ADJ as ssize_t {
                        le = le.offset(1);
                        printf(b" %d\n\x00" as *const u8 as *const c_char, *le);
                    } else {
                        printf(b"\n\x00" as *const u8 as *const c_char);
                    }
                }
            }
            line += 1
        } else if tk == '#' as ssize_t {
            while *p as c_int != 0 as c_int && *p as c_int != '\n' as i32 {
                p = p.offset(1)
            }
        } else if tk >= 'a' as ssize_t && tk <= 'z' as ssize_t || tk >= 'A' as ssize_t && tk <= 'Z' as ssize_t || tk == '_' as ssize_t {
            pp = p.offset(-1);
            while *p as c_int >= 'a' as i32 && *p as c_int <= 'z' as i32 || *p as c_int >= 'A' as i32 && *p as c_int <= 'Z' as i32 || *p as c_int >= '0' as i32 && *p as c_int <= '9' as i32 || *p as c_int == '_' as i32 {
                tk = tk * 147 as ssize_t + *p as ssize_t;
                p = p.offset(1);
            }
            tk = (tk << 6 as c_int) + p.wrapping_offset_from(pp) as ssize_t;
            id = sym;
            while *id.offset(Tk as ssize_t) != 0 {
                if tk == *id.offset(Hash as ssize_t) && memcmp(*id.offset(Name as ssize_t) as *mut c_char as *const c_void, pp as *const c_void, p.wrapping_offset_from(pp) as ssize_t as size_t) == 0 {
                    tk = *id.offset(Tk as ssize_t);
                    return;
                }
                id = id.offset(Idsz as ssize_t)
            }
            *id.offset(Name as ssize_t) = pp as ssize_t;
            *id.offset(Hash as ssize_t) = tk;
            let ref mut tmp = *id.offset(Tk as ssize_t);
            *tmp = Id as ssize_t;
            tk = *tmp;
            return;
        } else if tk >= '0' as ssize_t && tk <= '9' as ssize_t {
            ival = tk - '0' as ssize_t;
            if ival != 0 {
                while *p as c_int >= '0' as i32 && *p as c_int <= '9' as i32 {
                    ival = ival * 10 as ssize_t + *p as ssize_t - '0' as ssize_t;
                    p = p.offset(1);
                }
            } else if *p as c_int == 'x' as i32 || *p as c_int == 'X' as i32 {
                loop {
                    p = p.offset(1);
                    tk = *p as ssize_t;
                    if !(tk != 0 && (tk >= '0' as ssize_t && tk <= '9' as ssize_t || tk >= 'a' as ssize_t && tk <= 'f' as ssize_t || tk >= 'A' as ssize_t && tk <= 'F' as ssize_t)) {
                        break;
                    }
                    ival = ival * 16 as ssize_t + (tk & 15 as ssize_t) + (if tk >= 'A' as ssize_t { 9 as c_int } else { 0 as c_int }) as ssize_t
                }
            } else {
                while *p as c_int >= '0' as i32 && *p as c_int <= '7' as i32 {
                    ival = ival * 8 as ssize_t + *p as ssize_t - '0' as ssize_t;
                    p = p.offset(1);
                }
            }
            tk = Num as ssize_t;
            return;
        } else if tk == '/' as ssize_t {
            if *p as c_int == '/' as i32 {
                p = p.offset(1);
                while *p as c_int != 0 as c_int && *p as c_int != '\n' as i32 {
                    p = p.offset(1)
                }
            } else {
                tk = Div as ssize_t;
                return;
            }
        } else if tk == '\'' as ssize_t || tk == '\"' as ssize_t {
            pp = data;
            while *p as c_int != 0 as c_int && *p as ssize_t != tk {
                ival = *p as ssize_t;
                p = p.offset(1);
                if ival == '\\' as ssize_t {
                    ival = *p as ssize_t;
                    p = p.offset(1);
                    if ival == 'n' as ssize_t {
                        ival = '\n' as ssize_t
                    }
                }
                if tk == '\"' as ssize_t {
                    *data = ival as c_char;
                    data = data.offset(1);
                }
            }
            p = p.offset(1);
            if tk == '\"' as ssize_t {
                ival = pp as ssize_t
            } else {
                tk = Num as ssize_t
            }
            return;
        } else if tk == '=' as ssize_t {
            if *p as c_int == '=' as i32 {
                p = p.offset(1);
                tk = Eq as ssize_t
            } else {
                tk = Assign as ssize_t
            }
            return;
        } else if tk == '+' as ssize_t {
            if *p as c_int == '+' as i32 {
                p = p.offset(1);
                tk = Inc as ssize_t
            } else {
                tk = Add as ssize_t
            }
            return;
        } else if tk == '-' as ssize_t {
            if *p as c_int == '-' as i32 {
                p = p.offset(1);
                tk = Dec as ssize_t
            } else {
                tk = Sub as ssize_t
            }
            return;
        } else if tk == '!' as ssize_t {
            if *p as c_int == '=' as i32 {
                p = p.offset(1);
                tk = Ne as ssize_t
            }
            return;
        } else if tk == '<' as ssize_t {
            if *p as c_int == '=' as i32 {
                p = p.offset(1);
                tk = Le as ssize_t
            } else if *p as c_int == '<' as i32 {
                p = p.offset(1);
                tk = Shl as ssize_t
            } else {
                tk = Lt as ssize_t
            }
            return;
        } else if tk == '>' as ssize_t {
            if *p as c_int == '=' as i32 {
                p = p.offset(1);
                tk = Ge as ssize_t
            } else if *p as c_int == '>' as i32 {
                p = p.offset(1);
                tk = Shr as ssize_t
            } else {
                tk = Gt as ssize_t
            }
            return;
        } else if tk == '|' as ssize_t {
            if *p as c_int == '|' as i32 {
                p = p.offset(1);
                tk = Lor as ssize_t
            } else {
                tk = Or as ssize_t
            }
            return;
        } else if tk == '&' as ssize_t {
            if *p as c_int == '&' as i32 {
                p = p.offset(1);
                tk = Lan as ssize_t
            } else {
                tk = And as ssize_t
            }
            return;
        } else if tk == '^' as ssize_t {
            tk = Xor as ssize_t;
            return;
        } else if tk == '%' as ssize_t {
            tk = Mod as ssize_t;
            return;
        } else if tk == '*' as ssize_t {
            tk = Mul as ssize_t;
            return;
        } else if tk == '[' as ssize_t {
            tk = Brak as ssize_t;
            return;
        } else if tk == '?' as ssize_t {
            tk = Cond as ssize_t;
            return;
        } else if tk == '~' as ssize_t || tk == ';' as ssize_t || tk == '{' as ssize_t || tk == '}' as ssize_t || tk == '(' as ssize_t || tk == ')' as ssize_t || tk == ']' as ssize_t || tk == ',' as ssize_t || tk == ':' as ssize_t {
            return;
        }
    }
}

pub unsafe fn expr(lev: ssize_t) {
    let mut t: ssize_t = 0;
    let mut d: *mut ssize_t = 0 as *mut ssize_t;
    if tk == 0 {
        printf(b"%d: unexpected eof in expression\n\x00" as *const u8 as *const c_char, line);
        exit(-1);
    } else {
        if tk == Num as ssize_t {
            e = e.offset(1);
            *e = IMM as ssize_t;
            e = e.offset(1);
            *e = ival;
            next();
            ty = INT as ssize_t
        } else if tk == '\"' as ssize_t {
            e = e.offset(1);
            *e = IMM as ssize_t;
            e = e.offset(1);
            *e = ival;
            next();
            while tk == '\"' as ssize_t {
                next();
            }
            data = ((data as ssize_t as size_t).wrapping_add(std::mem::size_of::<ssize_t>() as size_t) & (std::mem::size_of::<ssize_t>() as size_t).wrapping_neg()) as *mut c_char;
            ty = PTR as ssize_t
        } else if tk == Sizeof as ssize_t {
            next();
            if tk == '(' as ssize_t {
                next();
            } else {
                printf(b"%d: open paren expected in sizeof\n\x00" as *const u8 as *const c_char, line);
                exit(-1);
            }
            ty = INT as ssize_t;
            if tk == Int as ssize_t {
                next();
            } else if tk == Char as ssize_t {
                next();
                ty = CHAR as ssize_t
            }
            while tk == Mul as ssize_t {
                next();
                ty = ty + PTR as ssize_t
            }
            if tk == ')' as ssize_t {
                next();
            } else {
                printf(b"%d: close paren expected in sizeof\n\x00" as *const u8 as *const c_char, line);
                exit(-1);
            }
            e = e.offset(1);
            *e = IMM as ssize_t;
            e = e.offset(1);
            *e = if ty == CHAR as ssize_t { std::mem::size_of::<c_char>() as size_t } else { std::mem::size_of::<ssize_t>() as size_t } as ssize_t;
            ty = INT as ssize_t
        } else if tk == Id as ssize_t {
            d = id;
            next();
            if tk == '(' as ssize_t {
                next();
                t = 0 as ssize_t;
                while tk != ')' as ssize_t {
                    expr(Assign as ssize_t);
                    e = e.offset(1);
                    *e = PSH as ssize_t;
                    t += 1;
                    if tk == ',' as ssize_t {
                        next();
                    }
                }
                next();
                if *d.offset(Class as ssize_t) == Sys as ssize_t {
                    e = e.offset(1);
                    *e = *d.offset(Val as ssize_t)
                } else if *d.offset(Class as ssize_t) == Fun as ssize_t {
                    e = e.offset(1);
                    *e = JSR as ssize_t;
                    e = e.offset(1);
                    *e = *d.offset(Val as ssize_t)
                } else {
                    printf(b"%d: bad function call\n\x00" as *const u8 as *const c_char, line);
                    exit(-1);
                }
                if t != 0 {
                    e = e.offset(1);
                    *e = ADJ as ssize_t;
                    e = e.offset(1);
                    *e = t
                }
                ty = *d.offset(Type as ssize_t)
            } else if *d.offset(Class as ssize_t) == Num as ssize_t {
                e = e.offset(1);
                *e = IMM as ssize_t;
                e = e.offset(1);
                *e = *d.offset(Val as ssize_t);
                ty = INT as ssize_t
            } else {
                if *d.offset(Class as ssize_t) == Loc as ssize_t {
                    e = e.offset(1);
                    *e = LEA as ssize_t;
                    e = e.offset(1);
                    *e = loc - *d.offset(Val as ssize_t)
                } else if *d.offset(Class as ssize_t) == Glo as ssize_t {
                    e = e.offset(1);
                    *e = IMM as ssize_t;
                    e = e.offset(1);
                    *e = *d.offset(Val as ssize_t)
                } else {
                    printf(b"%d: undefined variable\n\x00" as *const u8 as *const c_char, line);
                    exit(-1);
                }
                ty = *d.offset(Type as ssize_t);
                e = e.offset(1);
                *e = if ty == CHAR as ssize_t { LC as c_int } else { LI as c_int } as ssize_t
            }
        } else if tk == '(' as ssize_t {
            next();
            if tk == Int as ssize_t || tk == Char as ssize_t {
                t = if tk == Int as ssize_t { INT as c_int } else { CHAR as c_int } as ssize_t;
                next();
                while tk == Mul as ssize_t {
                    next();
                    t = t + PTR as ssize_t
                }
                if tk == ')' as ssize_t {
                    next();
                } else {
                    printf(b"%d: bad cast\n\x00" as *const u8 as *const c_char, line);
                    exit(-1);
                }
                expr(Inc as ssize_t);
                ty = t
            } else {
                expr(Assign as ssize_t);
                if tk == ')' as ssize_t {
                    next();
                } else {
                    printf(b"%d: close paren expected\n\x00" as *const u8 as *const c_char, line);
                    exit(-1);
                }
            }
        } else if tk == Mul as ssize_t {
            next();
            expr(Inc as ssize_t);
            if ty > INT as ssize_t {
                ty = ty - PTR as ssize_t
            } else {
                printf(b"%d: bad dereference\n\x00" as *const u8 as *const c_char, line);
                exit(-1);
            }
            e = e.offset(1);
            *e = if ty == CHAR as ssize_t { LC as c_int } else { LI as c_int } as ssize_t
        } else if tk == And as ssize_t {
            next();
            expr(Inc as ssize_t);
            if *e == LC as ssize_t || *e == LI as ssize_t {
                e = e.offset(-1)
            } else {
                printf(b"%d: bad address-of\n\x00" as *const u8 as *const c_char, line);
                exit(-1);
            }
            ty = ty + PTR as ssize_t
        } else if tk == '!' as ssize_t {
            next();
            expr(Inc as ssize_t);
            e = e.offset(1);
            *e = PSH as ssize_t;
            e = e.offset(1);
            *e = IMM as ssize_t;
            e = e.offset(1);
            *e = 0 as ssize_t;
            e = e.offset(1);
            *e = EQ as ssize_t;
            ty = INT as ssize_t
        } else if tk == '~' as ssize_t {
            next();
            expr(Inc as ssize_t);
            e = e.offset(1);
            *e = PSH as ssize_t;
            e = e.offset(1);
            *e = IMM as ssize_t;
            e = e.offset(1);
            *e = -1 as ssize_t;
            e = e.offset(1);
            *e = XOR as ssize_t;
            ty = INT as ssize_t
        } else if tk == Add as ssize_t {
            next();
            expr(Inc as ssize_t);
            ty = INT as ssize_t
        } else if tk == Sub as ssize_t {
            next();
            e = e.offset(1);
            *e = IMM as ssize_t;
            if tk == Num as ssize_t {
                e = e.offset(1);
                *e = -ival;
                next();
            } else {
                e = e.offset(1);
                *e = -1 as ssize_t;
                e = e.offset(1);
                *e = PSH as ssize_t;
                expr(Inc as ssize_t);
                e = e.offset(1);
                *e = MUL as ssize_t
            }
            ty = INT as ssize_t
        } else if tk == Inc as ssize_t || tk == Dec as ssize_t {
            t = tk;
            next();
            expr(Inc as ssize_t);
            if *e == LC as ssize_t {
                *e = PSH as ssize_t;
                e = e.offset(1);
                *e = LC as ssize_t
            } else if *e == LI as ssize_t {
                *e = PSH as ssize_t;
                e = e.offset(1);
                *e = LI as ssize_t
            } else {
                printf(b"%d: bad lvalue in pre-increment\n\x00" as *const u8 as *const c_char, line);
                exit(-1);
            }
            e = e.offset(1);
            *e = PSH as ssize_t;
            e = e.offset(1);
            *e = IMM as ssize_t;
            e = e.offset(1);
            *e = if ty > PTR as ssize_t { std::mem::size_of::<ssize_t>() as size_t } else { std::mem::size_of::<c_char>() as size_t } as ssize_t;
            e = e.offset(1);
            *e = if t == Inc as ssize_t { ADD as c_int } else { SUB as c_int } as ssize_t;
            e = e.offset(1);
            *e = if ty == CHAR as ssize_t { SC as c_int } else { SI as c_int } as ssize_t
        } else {
            printf(b"%d: bad expression\n\x00" as *const u8 as *const c_char, line);
            exit(-1);
        }
    }
    while tk >= lev {
        // "precedence climbing" or "Top Down Operator Precedence" method
        t = ty; // vm registers
        if tk == Assign as ssize_t {
            next(); // temps
            if *e == LC as ssize_t || *e == LI as ssize_t {
                *e = PSH as ssize_t
            } else {
                printf(b"%d: bad lvalue in assignment\n\x00" as *const u8 as *const c_char, line); // arbitrary size
                exit(-1); // add keywords to symbol table
            } // add library to symbol table
            expr(Assign as ssize_t); // handle void type
            ty = t; // keep track of main
            e = e.offset(1);
            *e = if ty == CHAR as ssize_t { SC as c_int } else { SI as c_int } as ssize_t
        } else if tk == Cond as ssize_t {
            next();
            e = e.offset(1);
            *e = BZ as ssize_t;
            e = e.offset(1);
            d = e;
            expr(Assign as ssize_t);
            if tk == ':' as ssize_t {
                next();
            } else {
                printf(b"%d: conditional missing colon\n\x00" as *const u8 as *const c_char, line);
                exit(-1);
            }
            *d = e.offset(3 as ssize_t) as ssize_t;
            e = e.offset(1);
            *e = JMP as ssize_t;
            e = e.offset(1);
            d = e;
            expr(Cond as ssize_t);
            *d = e.offset(1 as ssize_t) as ssize_t
        } else if tk == Lor as ssize_t {
            next();
            e = e.offset(1);
            *e = BNZ as ssize_t;
            e = e.offset(1);
            d = e;
            expr(Lan as ssize_t);
            *d = e.offset(1 as ssize_t) as ssize_t;
            ty = INT as ssize_t
        } else if tk == Lan as ssize_t {
            next();
            e = e.offset(1);
            *e = BZ as ssize_t;
            e = e.offset(1);
            d = e;
            expr(Or as ssize_t);
            *d = e.offset(1 as ssize_t) as ssize_t;
            ty = INT as ssize_t
        } else if tk == Or as ssize_t {
            next();
            e = e.offset(1);
            *e = PSH as ssize_t;
            expr(Xor as ssize_t);
            e = e.offset(1);
            *e = OR as ssize_t;
            ty = INT as ssize_t
        } else if tk == Xor as ssize_t {
            next();
            e = e.offset(1);
            *e = PSH as ssize_t;
            expr(And as ssize_t);
            e = e.offset(1);
            *e = XOR as ssize_t;
            ty = INT as ssize_t
        } else if tk == And as ssize_t {
            next();
            e = e.offset(1);
            *e = PSH as ssize_t;
            expr(Eq as ssize_t);
            e = e.offset(1);
            *e = AND as ssize_t;
            ty = INT as ssize_t
        } else if tk == Eq as ssize_t {
            next();
            e = e.offset(1);
            *e = PSH as ssize_t;
            expr(Lt as ssize_t);
            e = e.offset(1);
            *e = EQ as ssize_t;
            ty = INT as ssize_t
        } else if tk == Ne as ssize_t {
            next();
            e = e.offset(1);
            *e = PSH as ssize_t;
            expr(Lt as ssize_t);
            e = e.offset(1);
            *e = NE as ssize_t;
            ty = INT as ssize_t
        } else if tk == Lt as ssize_t {
            next();
            e = e.offset(1);
            *e = PSH as ssize_t;
            expr(Shl as ssize_t);
            e = e.offset(1);
            *e = LT as ssize_t;
            ty = INT as ssize_t
        } else if tk == Gt as ssize_t {
            next();
            e = e.offset(1);
            *e = PSH as ssize_t;
            expr(Shl as ssize_t);
            e = e.offset(1);
            *e = GT as ssize_t;
            ty = INT as ssize_t
        } else if tk == Le as ssize_t {
            next();
            e = e.offset(1);
            *e = PSH as ssize_t;
            expr(Shl as ssize_t);
            e = e.offset(1);
            *e = LE as ssize_t;
            ty = INT as ssize_t
        } else if tk == Ge as ssize_t {
            next();
            e = e.offset(1);
            *e = PSH as ssize_t;
            expr(Shl as ssize_t);
            e = e.offset(1);
            *e = GE as ssize_t;
            ty = INT as ssize_t
        } else if tk == Shl as ssize_t {
            next();
            e = e.offset(1);
            *e = PSH as ssize_t;
            expr(Add as ssize_t);
            e = e.offset(1);
            *e = SHL as ssize_t;
            ty = INT as ssize_t
        } else if tk == Shr as ssize_t {
            next();
            e = e.offset(1);
            *e = PSH as ssize_t;
            expr(Add as ssize_t);
            e = e.offset(1);
            *e = SHR as ssize_t;
            ty = INT as ssize_t
        } else if tk == Add as ssize_t {
            next();
            e = e.offset(1);
            *e = PSH as ssize_t;
            expr(Mul as ssize_t);
            ty = t;
            if ty > PTR as ssize_t {
                e = e.offset(1);
                *e = PSH as ssize_t;
                e = e.offset(1);
                *e = IMM as ssize_t;
                e = e.offset(1);
                *e = std::mem::size_of::<ssize_t>() as size_t as ssize_t;
                e = e.offset(1);
                *e = MUL as ssize_t
            }
            e = e.offset(1);
            *e = ADD as ssize_t
        } else if tk == Sub as ssize_t {
            next();
            e = e.offset(1);
            *e = PSH as ssize_t;
            expr(Mul as ssize_t);
            if t > PTR as ssize_t && t == ty {
                e = e.offset(1);
                *e = SUB as ssize_t;
                e = e.offset(1);
                *e = PSH as ssize_t;
                e = e.offset(1);
                *e = IMM as ssize_t;
                e = e.offset(1);
                *e = std::mem::size_of::<ssize_t>() as size_t as ssize_t;
                e = e.offset(1);
                *e = DIV as ssize_t;
                ty = INT as ssize_t
            } else {
                ty = t;
                if ty > PTR as ssize_t {
                    e = e.offset(1);
                    *e = PSH as ssize_t;
                    e = e.offset(1);
                    *e = IMM as ssize_t;
                    e = e.offset(1);
                    *e = std::mem::size_of::<ssize_t>() as size_t as ssize_t;
                    e = e.offset(1);
                    *e = MUL as ssize_t;
                    e = e.offset(1);
                    *e = SUB as ssize_t
                } else {
                    e = e.offset(1);
                    *e = SUB as ssize_t
                }
            }
        } else if tk == Mul as ssize_t {
            next();
            e = e.offset(1);
            *e = PSH as ssize_t;
            expr(Inc as ssize_t);
            e = e.offset(1);
            *e = MUL as ssize_t;
            ty = INT as ssize_t
        } else if tk == Div as ssize_t {
            next();
            e = e.offset(1);
            *e = PSH as ssize_t;
            expr(Inc as ssize_t);
            e = e.offset(1);
            *e = DIV as ssize_t;
            ty = INT as ssize_t
        } else if tk == Mod as ssize_t {
            next();
            e = e.offset(1);
            *e = PSH as ssize_t;
            expr(Inc as ssize_t);
            e = e.offset(1);
            *e = MOD as ssize_t;
            ty = INT as ssize_t
        } else if tk == Inc as ssize_t || tk == Dec as ssize_t {
            if *e == LC as ssize_t {
                *e = PSH as ssize_t;
                e = e.offset(1);
                *e = LC as ssize_t
            } else if *e == LI as ssize_t {
                *e = PSH as ssize_t;
                e = e.offset(1);
                *e = LI as ssize_t
            } else {
                printf(b"%d: bad lvalue in post-increment\n\x00" as *const u8 as *const c_char, line);
                exit(-1);
            }
            e = e.offset(1);
            *e = PSH as ssize_t;
            e = e.offset(1);
            *e = IMM as ssize_t;
            e = e.offset(1);
            *e = if ty > PTR as ssize_t { std::mem::size_of::<ssize_t>() as size_t } else { std::mem::size_of::<c_char>() as size_t } as ssize_t;
            e = e.offset(1);
            *e = if tk == Inc as ssize_t { ADD as c_int } else { SUB as c_int } as ssize_t;
            e = e.offset(1);
            *e = if ty == CHAR as ssize_t { SC as c_int } else { SI as c_int } as ssize_t;
            e = e.offset(1);
            *e = PSH as ssize_t;
            e = e.offset(1);
            *e = IMM as ssize_t;
            e = e.offset(1);
            *e = if ty > PTR as ssize_t { std::mem::size_of::<ssize_t>() as size_t } else { std::mem::size_of::<c_char>() as size_t } as ssize_t;
            e = e.offset(1);
            *e = if tk == Inc as ssize_t { SUB as c_int } else { ADD as c_int } as ssize_t;
            next();
        } else if tk == Brak as ssize_t {
            next();
            e = e.offset(1);
            *e = PSH as ssize_t;
            expr(Assign as ssize_t);
            if tk == ']' as ssize_t {
                next();
            } else {
                printf(b"%d: close bracket expected\n\x00" as *const u8 as *const c_char, line);
                exit(-1);
            }
            if t > PTR as ssize_t {
                e = e.offset(1);
                *e = PSH as ssize_t;
                e = e.offset(1);
                *e = IMM as ssize_t;
                e = e.offset(1);
                *e = std::mem::size_of::<ssize_t>() as size_t as ssize_t;
                e = e.offset(1);
                *e = MUL as ssize_t
            } else if t < PTR as ssize_t {
                printf(b"%d: pointer type expected\n\x00" as *const u8 as *const c_char, line);
                exit(-1);
            }
            e = e.offset(1);
            *e = ADD as ssize_t;
            ty = t - PTR as ssize_t;
            e = e.offset(1);
            *e = if ty == CHAR as ssize_t { LC as c_int } else { LI as c_int } as ssize_t
        } else {
            printf(b"%d: compiler error tk=%d\n\x00" as *const u8 as *const c_char, line, tk);
            exit(-1);
        }
    }
}

pub unsafe fn stmt() {
    let mut a: *mut ssize_t = 0 as *mut ssize_t;
    let mut b: *mut ssize_t = 0 as *mut ssize_t;
    if tk == If as ssize_t {
        next();
        if tk == '(' as ssize_t {
            next();
        } else {
            printf(b"%d: open paren expected\n\x00" as *const u8 as *const c_char, line);
            exit(-1);
        }
        expr(Assign as ssize_t);
        if tk == ')' as ssize_t {
            next();
        } else {
            printf(b"%d: close paren expected\n\x00" as *const u8 as *const c_char, line);
            exit(-1);
        }
        e = e.offset(1);
        *e = BZ as ssize_t;
        e = e.offset(1);
        b = e;
        stmt();
        if tk == Else as ssize_t {
            *b = e.offset(3 as ssize_t) as ssize_t;
            e = e.offset(1);
            *e = JMP as ssize_t;
            e = e.offset(1);
            b = e;
            next();
            stmt();
        }
        *b = e.offset(1 as ssize_t) as ssize_t
    } else if tk == While as ssize_t {
        next();
        a = e.offset(1 as ssize_t);
        if tk == '(' as ssize_t {
            next();
        } else {
            printf(b"%d: open paren expected\n\x00" as *const u8 as *const c_char, line);
            exit(-1);
        }
        expr(Assign as ssize_t);
        if tk == ')' as ssize_t {
            next();
        } else {
            printf(b"%d: close paren expected\n\x00" as *const u8 as *const c_char, line);
            exit(-1);
        }
        e = e.offset(1);
        *e = BZ as ssize_t;
        e = e.offset(1);
        b = e;
        stmt();
        e = e.offset(1);
        *e = JMP as ssize_t;
        e = e.offset(1);
        *e = a as ssize_t;
        *b = e.offset(1 as ssize_t) as ssize_t
    } else if tk == Return as ssize_t {
        next();
        if tk != ';' as ssize_t {
            expr(Assign as ssize_t);
        }
        e = e.offset(1);
        *e = LEV as ssize_t;
        if tk == ';' as ssize_t {
            next();
        } else {
            printf(b"%d: semicolon expected\n\x00" as *const u8 as *const c_char, line);
            exit(-1);
        }
    } else if tk == '{' as ssize_t {
        next();
        while tk != '}' as ssize_t {
            stmt();
        }
        next();
    } else if tk == ';' as ssize_t {
        next();
    } else {
        expr(Assign as ssize_t);
        if tk == ';' as ssize_t {
            next();
        } else {
            printf(b"%d: semicolon expected\n\x00" as *const u8 as *const c_char, line);
            exit(-1);
        }
    };
}

pub fn main() { unsafe {
    let mut args: Vec<*mut c_char> = Vec::new();
    for arg in std::env::args() {
        args.push(std::ffi::CString::new(arg).expect("Failed to convert argument into CString.").into_raw());
    }
    args.push(std::ptr::null_mut());

    let mut argc = (args.len() - 1) as ssize_t;
    let mut argv = args.as_mut_ptr() as *mut *mut c_char;

    let mut fd: ssize_t = 0;
    let mut bt: ssize_t = 0;
    let mut ty_0: ssize_t = 0;
    let mut poolsz: ssize_t = 0;
    let mut idmain: *mut ssize_t = 0 as *mut ssize_t;
    let mut pc: *mut ssize_t = 0 as *mut ssize_t;
    let mut sp: *mut ssize_t = 0 as *mut ssize_t;
    let mut bp: *mut ssize_t = 0 as *mut ssize_t;
    let mut a: ssize_t = 0;
    let mut cycle: ssize_t = 0;
    let mut i: ssize_t = 0;
    let mut t: *mut ssize_t = 0 as *mut ssize_t;
    argc -= 1;
    argv = argv.offset(1);
    if argc > 0 as ssize_t && **argv as c_int == '-' as i32 && *(*argv).offset(1 as ssize_t) as c_int == 's' as i32 {
        src = 1 as ssize_t;
        argc -= 1;
        argv = argv.offset(1)
    }
    if argc > 0 as ssize_t && **argv as c_int == '-' as i32 && *(*argv).offset(1 as ssize_t) as c_int == 'd' as i32 {
        debug = 1 as ssize_t;
        argc -= 1;
        argv = argv.offset(1)
    }
    if argc < 1 as ssize_t {
        printf(b"usage: c4 [-s] [-d] file ...\n\x00" as *const u8 as *const c_char);
        exit(-1);
    }
    fd = open(*argv, 0 as c_int) as ssize_t;
    if fd < 0 as ssize_t {
        printf(b"could not open(%s)\n\x00" as *const u8 as *const c_char, *argv);
        exit(-1);
    }
    poolsz = (256 as c_int * 1024 as c_int) as ssize_t;
    sym = malloc(poolsz as size_t) as *mut ssize_t;
    if sym.is_null() {
        printf(b"could not malloc(%d) symbol area\n\x00" as *const u8 as *const c_char, poolsz);
        exit(-1);
    }
    e = malloc(poolsz as size_t) as *mut ssize_t;
    le = e;
    if le.is_null() {
        printf(b"could not malloc(%d) text area\n\x00" as *const u8 as *const c_char, poolsz);
        exit(-1);
    }
    data = malloc(poolsz as size_t) as *mut c_char;
    if data.is_null() {
        printf(b"could not malloc(%d) data area\n\x00" as *const u8 as *const c_char, poolsz);
        exit(-1);
    }
    sp = malloc(poolsz as size_t) as *mut ssize_t;
    if sp.is_null() {
        printf(b"could not malloc(%d) stack area\n\x00" as *const u8 as *const c_char, poolsz);
        exit(-1);
    }
    memset(sym as *mut c_void, 0 as c_int, poolsz as size_t);
    memset(e as *mut c_void, 0 as c_int, poolsz as size_t);
    memset(data as *mut c_void, 0 as c_int, poolsz as size_t);
    p = b"char else enum if int return sizeof while open read close printf malloc free memset memcmp exit void main\x00" as *const u8 as *const c_char as *mut c_char;
    i = Char as ssize_t;
    while i <= While as ssize_t {
        next();
        *id.offset(Tk as ssize_t) = i;
        i = i + 1;
    }
    i = OPEN as ssize_t;
    while i <= EXIT as ssize_t {
        next();
        *id.offset(Class as ssize_t) = Sys as ssize_t;
        *id.offset(Type as ssize_t) = INT as ssize_t;
        *id.offset(Val as ssize_t) = i;
        i = i + 1;
    }
    next();
    *id.offset(Tk as ssize_t) = Char as ssize_t;
    next();
    idmain = id;
    p = malloc(poolsz as size_t) as *mut c_char;
    lp = p;
    if lp.is_null() {
        printf(b"could not malloc(%d) source area\n\x00" as *const u8 as *const c_char, poolsz);
        exit(-1);
    }
    i = read(fd as c_int, p as *mut c_void, (poolsz - 1 as ssize_t) as size_t);
    if i <= 0 as ssize_t {
        printf(b"read() returned %d\n\x00" as *const u8 as *const c_char, i);
        exit(-1);
    }
    *p.offset(i as isize) = 0 as c_int as c_char;
    close(fd as c_int);
    // parse declarations
    line = 1 as ssize_t; // basetype
    next();
    while tk != 0 {
        bt = INT as ssize_t;
        if tk == Int as ssize_t {
            next();
        } else if tk == Char as ssize_t {
            next();
            bt = CHAR as ssize_t
        } else if tk == Enum as ssize_t {
            next();
            if tk != '{' as ssize_t {
                next();
            }
            if tk == '{' as ssize_t {
                next();
                i = 0 as ssize_t;
                while tk != '}' as ssize_t {
                    if tk != Id as ssize_t {
                        printf(b"%d: bad enum identifier %d\n\x00" as *const u8 as *const c_char, line, tk);
                        exit(-1);
                    }
                    next();
                    if tk == Assign as ssize_t {
                        next();
                        if tk != Num as ssize_t {
                            printf(b"%d: bad enum initializer\n\x00" as *const u8 as *const c_char, line);
                            exit(-1);
                        }
                        i = ival;
                        next();
                    }
                    *id.offset(Class as ssize_t) = Num as ssize_t;
                    *id.offset(Type as ssize_t) = INT as ssize_t;
                    *id.offset(Val as ssize_t) = i;
                    i = i + 1;
                    if tk == ',' as ssize_t {
                        next();
                    }
                }
                next();
            }
        }
        while tk != ';' as ssize_t && tk != '}' as ssize_t {
            ty_0 = bt;
            while tk == Mul as ssize_t {
                next();
                ty_0 = ty_0 + PTR as ssize_t
            }
            if tk != Id as ssize_t {
                printf(b"%d: bad global declaration\n\x00" as *const u8 as *const c_char, line);
                exit(-1);
            }
            if *id.offset(Class as ssize_t) != 0 {
                printf(b"%d: duplicate global definition\n\x00" as *const u8 as *const c_char, line);
                exit(-1);
            }
            next();
            *id.offset(Type as ssize_t) = ty_0;
            if tk == '(' as ssize_t {
                // function
                *id.offset(Class as ssize_t) = Fun as ssize_t; // unwind symbol table locals
                *id.offset(Val as ssize_t) = e.offset(1 as ssize_t) as ssize_t;
                next();
                i = 0 as ssize_t;
                while tk != ')' as ssize_t {
                    ty_0 = INT as ssize_t;
                    if tk == Int as ssize_t {
                        next();
                    } else if tk == Char as ssize_t {
                        next();
                        ty_0 = CHAR as ssize_t
                    }
                    while tk == Mul as ssize_t {
                        next();
                        ty_0 = ty_0 + PTR as ssize_t
                    }
                    if tk != Id as ssize_t {
                        printf(b"%d: bad parameter declaration\n\x00" as *const u8 as *const c_char, line);
                        exit(-1);
                    }
                    if *id.offset(Class as ssize_t) == Loc as ssize_t {
                        printf(b"%d: duplicate parameter definition\n\x00" as *const u8 as *const c_char, line);
                        exit(-1);
                    }
                    *id.offset(HClass as ssize_t) = *id.offset(Class as ssize_t);
                    *id.offset(Class as ssize_t) = Loc as ssize_t;
                    *id.offset(HType as ssize_t) = *id.offset(Type as ssize_t);
                    *id.offset(Type as ssize_t) = ty_0;
                    *id.offset(HVal as ssize_t) = *id.offset(Val as ssize_t);
                    *id.offset(Val as ssize_t) = i;
                    i = i + 1;
                    next();
                    if tk == ',' as ssize_t {
                        next();
                    }
                }
                next();
                if tk != '{' as ssize_t {
                    printf(b"%d: bad function definition\n\x00" as *const u8 as *const c_char, line);
                    exit(-1);
                }
                i += 1;
                loc = i;
                next();
                while tk == Int as ssize_t || tk == Char as ssize_t {
                    bt = if tk == Int as ssize_t { INT as c_int } else { CHAR as c_int } as ssize_t;
                    next();
                    while tk != ';' as ssize_t {
                        ty_0 = bt;
                        while tk == Mul as ssize_t {
                            next();
                            ty_0 = ty_0 + PTR as ssize_t
                        }
                        if tk != Id as ssize_t {
                            printf(b"%d: bad local declaration\n\x00" as *const u8 as *const c_char, line);
                            exit(-1);
                        }
                        if *id.offset(Class as ssize_t) == Loc as ssize_t {
                            printf(b"%d: duplicate local definition\n\x00" as *const u8 as *const c_char, line);
                            exit(-1);
                        }
                        *id.offset(HClass as ssize_t) = *id.offset(Class as ssize_t);
                        *id.offset(Class as ssize_t) = Loc as ssize_t;
                        *id.offset(HType as ssize_t) = *id.offset(Type as ssize_t);
                        *id.offset(Type as ssize_t) = ty_0;
                        *id.offset(HVal as ssize_t) = *id.offset(Val as ssize_t);
                        i += 1;
                        *id.offset(Val as ssize_t) = i;
                        next();
                        if tk == ',' as ssize_t {
                            next();
                        }
                    }
                    next();
                }
                e = e.offset(1);
                *e = ENT as ssize_t;
                e = e.offset(1);
                *e = i - loc;
                while tk != '}' as ssize_t {
                    stmt();
                }
                e = e.offset(1);
                *e = LEV as ssize_t;
                id = sym;
                while *id.offset(Tk as ssize_t) != 0 {
                    if *id.offset(Class as ssize_t) == Loc as ssize_t {
                        *id.offset(Class as ssize_t) = *id.offset(HClass as ssize_t);
                        *id.offset(Type as ssize_t) = *id.offset(HType as ssize_t);
                        *id.offset(Val as ssize_t) = *id.offset(HVal as ssize_t)
                    }
                    id = id.offset(Idsz as ssize_t)
                }
            } else {
                *id.offset(Class as ssize_t) = Glo as ssize_t;
                *id.offset(Val as ssize_t) = data as ssize_t;
                data = data.offset(std::mem::size_of::<ssize_t>() as size_t as isize)
            }
            if tk == ',' as ssize_t {
                next();
            }
        }
        next();
    }
    pc = *idmain.offset(Val as ssize_t) as *mut ssize_t;
    if pc.is_null() {
        printf(b"main() not defined\n\x00" as *const u8 as *const c_char);
        exit(-1);
    }
    if src != 0 {
        exit(0);
    }
    // setup stack
    sp = (sp as ssize_t + poolsz) as *mut ssize_t; // call exit if main returns
    bp = sp;
    sp = sp.offset(-1);
    *sp = EXIT as ssize_t;
    sp = sp.offset(-1);
    *sp = PSH as ssize_t;
    t = sp;
    sp = sp.offset(-1);
    *sp = argc;
    sp = sp.offset(-1);
    *sp = argv as ssize_t;
    sp = sp.offset(-1);
    *sp = t as ssize_t;
    // run...
    cycle = 0 as ssize_t; // load local address
    loop {
        i = *pc; // jump to subroutine; // load global address or immediate
        pc = pc.offset(1); // jump
        cycle += 1; // branch if zero
        if debug != 0 {
            printf(
                b"%d> %.4s\x00" as *const u8 as *const c_char,
                cycle,
                &*(b"LEA ,IMM ,JMP ,JSR ,BZ  ,BNZ ,ENT ,ADJ ,LEV ,LI  ,LC  ,SI  ,SC  ,PSH ,OR  ,XOR ,AND ,EQ  ,NE  ,LT  ,GT  ,LE  ,GE  ,SHL ,SHR ,ADD ,SUB ,MUL ,DIV ,MOD ,OPEN,READ,CLOS,PRTF,MALC,FREE,MSET,MCMP,EXIT,\x00" as *const u8 as *const c_char).offset((i * 5 as c_int as ssize_t) as isize) as *const c_char,
            ); // branch if not zero
            if i <= ADJ as ssize_t {
                printf(b" %d\n\x00" as *const u8 as *const c_char, *pc); // enter subroutine
            } else {
                printf(b"\n\x00" as *const u8 as *const c_char); // stack adjust
            }
        } // leave subroutine
        if i == LEA as ssize_t {
            a = bp.offset(*pc as isize) as ssize_t; // load int
            pc = pc.offset(1); // load char
        } else if i == IMM as ssize_t {
            a = *pc; // store int
            pc = pc.offset(1); // store char
        } else if i == JMP as ssize_t {
            pc = *pc as *mut ssize_t
        } else if i == JSR as ssize_t {
            sp = sp.offset(-1); // push
            *sp = pc.offset(1 as ssize_t) as ssize_t;
            pc = *pc as *mut ssize_t
        } else if i == BZ as ssize_t {
            pc = if a != 0 { pc.offset(1 as ssize_t) } else { *pc as *mut ssize_t }
        } else if i == BNZ as ssize_t {
            pc = if a != 0 { *pc as *mut ssize_t } else { pc.offset(1 as ssize_t) }
        } else if i == ENT as ssize_t {
            sp = sp.offset(-1);
            *sp = bp as ssize_t;
            bp = sp;
            sp = sp.offset(-(*pc as isize));
            pc = pc.offset(1);
        } else if i == ADJ as ssize_t {
            sp = sp.offset(*pc as isize);
            pc = pc.offset(1);
        } else if i == LEV as ssize_t {
            sp = bp;
            bp = *sp as *mut ssize_t;
            sp = sp.offset(1);
            pc = *sp as *mut ssize_t;
            sp = sp.offset(1);
        } else if i == LI as ssize_t {
            a = *(a as *mut ssize_t)
        } else if i == LC as ssize_t {
            a = *(a as *mut c_char) as ssize_t
        } else if i == SI as ssize_t {
            *(*sp as *mut ssize_t) = a;
            sp = sp.offset(1);
        } else if i == SC as ssize_t {
            let ref mut tmp = *(*sp as *mut c_char);
            *tmp = a as c_char;
            a = *tmp as ssize_t;
            sp = sp.offset(1);
        } else if i == PSH as ssize_t {
            sp = sp.offset(-1);
            *sp = a
        } else if i == OR as ssize_t {
            a = *sp | a;
            sp = sp.offset(1);
        } else if i == XOR as ssize_t {
            a = *sp ^ a;
            sp = sp.offset(1);
        } else if i == AND as ssize_t {
            a = *sp & a;
            sp = sp.offset(1);
        } else if i == EQ as ssize_t {
            a = (*sp == a) as ssize_t;
            sp = sp.offset(1);
        } else if i == NE as ssize_t {
            a = (*sp != a) as ssize_t;
            sp = sp.offset(1);
        } else if i == LT as ssize_t {
            a = (*sp < a) as ssize_t;
            sp = sp.offset(1);
        } else if i == GT as ssize_t {
            a = (*sp > a) as ssize_t;
            sp = sp.offset(1);
        } else if i == LE as ssize_t {
            a = (*sp <= a) as ssize_t;
            sp = sp.offset(1);
        } else if i == GE as ssize_t {
            a = (*sp >= a) as ssize_t;
            sp = sp.offset(1);
        } else if i == SHL as ssize_t {
            a = *sp << a;
            sp = sp.offset(1);
        } else if i == SHR as ssize_t {
            a = *sp >> a;
            sp = sp.offset(1);
        } else if i == ADD as ssize_t {
            a = *sp + a;
            sp = sp.offset(1);
        } else if i == SUB as ssize_t {
            a = *sp - a;
            sp = sp.offset(1);
        } else if i == MUL as ssize_t {
            a = *sp * a;
            sp = sp.offset(1);
        } else if i == DIV as ssize_t {
            a = *sp / a;
            sp = sp.offset(1);
        } else if i == MOD as ssize_t {
            a = *sp % a;
            sp = sp.offset(1);
        } else if i == OPEN as ssize_t {
            a = open(*sp.offset(1 as ssize_t) as *mut c_char, *sp as c_int) as ssize_t
        } else if i == READ as ssize_t {
            a = read(*sp.offset(2 as ssize_t) as c_int, *sp.offset(1 as ssize_t) as *mut c_char as *mut c_void, *sp as size_t)
        } else if i == CLOS as ssize_t {
            a = close(*sp as c_int) as ssize_t
        } else if i == PRTF as ssize_t {
            t = sp.offset(*pc.offset(1 as ssize_t) as isize);
            a = printf(*t.offset(-1) as *mut c_char, *t.offset(-2), *t.offset(-3), *t.offset(-4), *t.offset(-5), *t.offset(-6)) as ssize_t
        } else if i == MALC as ssize_t {
            a = malloc(*sp as size_t) as ssize_t
        } else if i == FREE as ssize_t {
            free(*sp as *mut c_void);
        } else if i == MSET as ssize_t {
            a = memset(*sp.offset(2 as ssize_t) as *mut c_char as *mut c_void, *sp.offset(1 as ssize_t) as c_int, *sp as size_t) as ssize_t
        } else if i == MCMP as ssize_t {
            a = memcmp(*sp.offset(2 as ssize_t) as *mut c_char as *const c_void, *sp.offset(1 as ssize_t) as *mut c_char as *const c_void, *sp as size_t) as ssize_t
        } else if i == EXIT as ssize_t {
            printf(b"exit(%d) cycle = %d\n\x00" as *const u8 as *const c_char, *sp, cycle);
            exit(*sp as i32);
        } else {
            printf(b"unknown instruction = %d! cycle = %d\n\x00" as *const u8 as *const c_char, i, cycle);
            exit(-1);
        }
    }
}}
