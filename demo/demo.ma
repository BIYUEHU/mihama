import { Show, Eq } from "prelude"
import as std from "stdlib"

// Data type definitions
type Bool = True | False
type Nat = Z | S(Nat)
type Nat2 =
 | Z2
 | S2 => Nat2
type Pair = <A, B> MkPair(A, B)
type Maybe = <A> Just(A) | Nothing
type Result = <A, E> Ok(A) | Err(E)
export abstract type Option = <A> Some(A) | None
export type List = <A> Nil | Cons(A, List(A))
type Vector = <L: Nat, A: Kind>
 | VNil => Vector(Z, A)
 | VCons => A => Vector(L, A) => Vector(S(L),A)

// Type aliases
export let String = List(Char)
let Array = List
let ShowFn = (A: Kind) => A -> String
let IntToStringFn = Int -> String

// Common List functions with correct type declarations
export let map: <A, B> (A -> B) -> List(A) -> List(B) =
    | Nil => Nil
    | Cons(x, xs) => Cons(f(x), map(f, xs))

export let foldr: <A, B> (A -> B -> B) -> B -> List(A) -> B =
    | Nil => acc => acc
    | Cons(x, xs) => acc => f(x)(foldr(f)(acc)(xs))

export let foldl: <A, B> (B -> A -> B) -> B -> List(A) -> B =
    | Nil => acc => acc
    | Cons(x, xs) => acc => foldl(f)(f(acc)(x))(xs)

let append: <A> List(A) -> List(A) -> List(A) =
    | Nil => ys => ys
    | Cons(x, xs) => ys => Cons(x, append(xs)(ys))

let filter: <A> (A -> Bool) -> List(A) -> List(A) =
    | Nil => Nil
    | Cons(x, xs) => match p(x) then
        | True => Cons(x, filter(p)(xs))
        | False => filter(p)(xs)

let reverse: <A> List(A) -> List(A) =
    | Nil => Nil
    | Cons(x, xs) => append(reverse(xs), Cons(x, Nil))

let compose: = <A, B, C>
    (f: B -> C, f: A -> B, x: A -C) => f(g(x))

let fst: Pair(A, B) -> A =
    | MkPair(x, _) => x

let snd: Pair(A, B) -> B =
    | MkPair(_, y) => y

// Let-in expression
let factorial: Nat -> Nat = n =>
    let helper: Nat -> Nat -> Nat =
        | Zero => acc => acc
        | Succ(n) => acc => helper(n, mul(Succ(n), acc))
    in helper(n, Succ(Zero))

// Match expression
let isZero: Nat -> Bool = n => match n then
    | Zero => True
    | Succ(_) => False

// Curried function
let add: Nat -> Nat -> Nat =
    | Zero => n => n
    | Succ(m) if m > 5 => n => Succ(add(m, n))
    | Succ(m) => m

let addFive: Nat -> Nat = add(5)

type State: Kind -> Kind -> Kind = <S, A> State(S -> (A, S))

map(["a", "b"], x => x + "!")

map<Int, String>([1, 2, 3], show)

let compute = {
    let x = 10
    let y = 20
    x + y
}

let compose: (B -> C) -> (A -> B) -> (A -> C) =
    (f, g) => x => f(g(x))

let fibonacci: Int -> Int =
    n => match n then
        | 1 => 1
        | 2 => 1
        | n => fibonacci(n) + fibonacci(n - 2)


let fibonacii2: Int -> Int =
 | 1 => 1
 | 2 => 1
 | n => fibonacci2(n - 1) + fibonacci2(n - 2)
