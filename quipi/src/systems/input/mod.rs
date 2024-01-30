pub struct Input {
    
}

#[derive(Debug, Hash, PartialEq, Eq, Clone, Copy)]
pub enum KeyState {
    Esc(bool),
    A(bool),
    B(bool),
    C(bool),
    D(bool),
    E(bool),
    F(bool),
    G(bool),
    H(bool),
    I(bool),
    J(bool),
    K(bool),
    L(bool),
    M(bool),
    N(bool),
    O(bool),
    P(bool),
    Q(bool),
    R(bool),
    S(bool),
    T(bool),
    U(bool),
    V(bool),
    W(bool),
    X(bool),
    Y(bool),
    Z(bool),
    N1(bool),
    N2(bool),
    N3(bool),
    N4(bool),
    N5(bool),
    N6(bool),
    N7(bool),
    N8(bool),
    N9(bool),
    N0(bool),
    Left(bool),
    Right(bool),
    Up(bool),
    Down(bool),
    Space(bool),
    LShift(bool),
    RShift(bool),
    LCtrl(bool),
    RCtrl(bool),
    LAlt(bool),
    RAlt(bool),
    Tab(bool),
    Caps(bool),
    Enter(bool),
}

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum MouseState {
    RBtn(bool),
    LBtn(bool),
    MBtn(bool),
    Pos((f32, f32, f32, f32)), // (x, y, relx, rely)
    Wheel(bool)
}
