// This code is part of Qiskit Rust bindings.
//
// (C) Copyright IBM 2025
//
// This code is licensed under the Apache License, Version 2.0. You may
// obtain a copy of this license in the LICENSE.txt file in the root directory
// of this source tree or at http://www.apache.org/licenses/LICENSE-2.0.
//
// Any modifications or derivative works of this code must retain this
// copyright notice, and modified files need to carry a notice indicating
// that they have been altered from the originals.

use qiskit_sys::{
    QkGate, QkGate_QkGate_C3SX, QkGate_QkGate_C3X, QkGate_QkGate_CCX, QkGate_QkGate_CCZ,
    QkGate_QkGate_CH, QkGate_QkGate_CPhase, QkGate_QkGate_CRX, QkGate_QkGate_CRY,
    QkGate_QkGate_CRZ, QkGate_QkGate_CS, QkGate_QkGate_CSX, QkGate_QkGate_CSdg,
    QkGate_QkGate_CSwap, QkGate_QkGate_CU, QkGate_QkGate_CU1, QkGate_QkGate_CU3, QkGate_QkGate_CX,
    QkGate_QkGate_CY, QkGate_QkGate_CZ, QkGate_QkGate_DCX, QkGate_QkGate_ECR,
    QkGate_QkGate_GlobalPhase, QkGate_QkGate_H, QkGate_QkGate_I, QkGate_QkGate_ISwap,
    QkGate_QkGate_Phase, QkGate_QkGate_R, QkGate_QkGate_RC3X, QkGate_QkGate_RCCX, QkGate_QkGate_RX,
    QkGate_QkGate_RXX, QkGate_QkGate_RY, QkGate_QkGate_RYY, QkGate_QkGate_RZ, QkGate_QkGate_RZX,
    QkGate_QkGate_RZZ, QkGate_QkGate_S, QkGate_QkGate_SX, QkGate_QkGate_SXdg, QkGate_QkGate_Sdg,
    QkGate_QkGate_Swap, QkGate_QkGate_T, QkGate_QkGate_Tdg, QkGate_QkGate_U, QkGate_QkGate_U1,
    QkGate_QkGate_U2, QkGate_QkGate_U3, QkGate_QkGate_X, QkGate_QkGate_XXMinusYY,
    QkGate_QkGate_XXPlusYY, QkGate_QkGate_Y, QkGate_QkGate_Z, qk_gate_num_params,
    qk_gate_num_qubits,
};

/// Representation of all of the compatible Standard Gates.
#[derive(Debug, Clone, Copy)]
#[repr(u8)]
pub enum StandardGate {
    GlobalPhase = QkGate_QkGate_GlobalPhase,
    H = QkGate_QkGate_H,
    I = QkGate_QkGate_I,
    X = QkGate_QkGate_X,
    Y = QkGate_QkGate_Y,
    Z = QkGate_QkGate_Z,
    Phase = QkGate_QkGate_Phase,
    R = QkGate_QkGate_R,
    RX = QkGate_QkGate_RX,
    RY = QkGate_QkGate_RY,
    RZ = QkGate_QkGate_RZ,
    S = QkGate_QkGate_S,
    Sdg = QkGate_QkGate_Sdg,
    SX = QkGate_QkGate_SX,
    SXdg = QkGate_QkGate_SXdg,
    T = QkGate_QkGate_T,
    Tdg = QkGate_QkGate_Tdg,
    U = QkGate_QkGate_U,
    U1 = QkGate_QkGate_U1,
    U2 = QkGate_QkGate_U2,
    U3 = QkGate_QkGate_U3,
    CH = QkGate_QkGate_CH,
    CX = QkGate_QkGate_CX,
    CY = QkGate_QkGate_CY,
    CZ = QkGate_QkGate_CZ,
    DCX = QkGate_QkGate_DCX,
    ECR = QkGate_QkGate_ECR,
    Swap = QkGate_QkGate_Swap,
    ISwap = QkGate_QkGate_ISwap,
    CPhase = QkGate_QkGate_CPhase,
    CRX = QkGate_QkGate_CRX,
    CRY = QkGate_QkGate_CRY,
    CRZ = QkGate_QkGate_CRZ,
    CS = QkGate_QkGate_CS,
    CSdg = QkGate_QkGate_CSdg,
    CSX = QkGate_QkGate_CSX,
    CU = QkGate_QkGate_CU,
    CU1 = QkGate_QkGate_CU1,
    CU3 = QkGate_QkGate_CU3,
    RXX = QkGate_QkGate_RXX,
    RYY = QkGate_QkGate_RYY,
    RZZ = QkGate_QkGate_RZZ,
    RZX = QkGate_QkGate_RZX,
    XXMinusYY = QkGate_QkGate_XXMinusYY,
    XXPlusYY = QkGate_QkGate_XXPlusYY,
    CCX = QkGate_QkGate_CCX,
    CCZ = QkGate_QkGate_CCZ,
    CSwap = QkGate_QkGate_CSwap,
    RCCX = QkGate_QkGate_RCCX,
    C3X = QkGate_QkGate_C3X,
    C3SX = QkGate_QkGate_C3SX,
    RC3X = QkGate_QkGate_RC3X,
}

impl From<QkGate> for StandardGate {
    #[allow(non_upper_case_globals)]
    fn from(value: QkGate) -> Self {
        match value {
            QkGate_QkGate_GlobalPhase => StandardGate::GlobalPhase,
            QkGate_QkGate_H => StandardGate::H,
            QkGate_QkGate_I => StandardGate::I,
            QkGate_QkGate_X => StandardGate::X,
            QkGate_QkGate_Y => StandardGate::Y,
            QkGate_QkGate_Z => StandardGate::Z,
            QkGate_QkGate_Phase => StandardGate::Phase,
            QkGate_QkGate_R => StandardGate::R,
            QkGate_QkGate_RX => StandardGate::RX,
            QkGate_QkGate_RY => StandardGate::RY,
            QkGate_QkGate_RZ => StandardGate::RZ,
            QkGate_QkGate_S => StandardGate::S,
            QkGate_QkGate_Sdg => StandardGate::Sdg,
            QkGate_QkGate_SX => StandardGate::SX,
            QkGate_QkGate_SXdg => StandardGate::SXdg,
            QkGate_QkGate_T => StandardGate::T,
            QkGate_QkGate_Tdg => StandardGate::Tdg,
            QkGate_QkGate_U => StandardGate::U,
            QkGate_QkGate_U1 => StandardGate::U1,
            QkGate_QkGate_U2 => StandardGate::U2,
            QkGate_QkGate_U3 => StandardGate::U3,
            QkGate_QkGate_CH => StandardGate::CH,
            QkGate_QkGate_CX => StandardGate::CX,
            QkGate_QkGate_CY => StandardGate::CY,
            QkGate_QkGate_CZ => StandardGate::CZ,
            QkGate_QkGate_DCX => StandardGate::DCX,
            QkGate_QkGate_ECR => StandardGate::ECR,
            QkGate_QkGate_Swap => StandardGate::Swap,
            QkGate_QkGate_ISwap => StandardGate::ISwap,
            QkGate_QkGate_CPhase => StandardGate::CPhase,
            QkGate_QkGate_CRX => StandardGate::CRX,
            QkGate_QkGate_CRY => StandardGate::CRY,
            QkGate_QkGate_CRZ => StandardGate::CRZ,
            QkGate_QkGate_CS => StandardGate::CS,
            QkGate_QkGate_CSdg => StandardGate::CSdg,
            QkGate_QkGate_CSX => StandardGate::CSX,
            QkGate_QkGate_CU => StandardGate::CU,
            QkGate_QkGate_CU1 => StandardGate::CU1,
            QkGate_QkGate_CU3 => StandardGate::CU3,
            QkGate_QkGate_RXX => StandardGate::RXX,
            QkGate_QkGate_RYY => StandardGate::RYY,
            QkGate_QkGate_RZZ => StandardGate::RZZ,
            QkGate_QkGate_RZX => StandardGate::RZX,
            QkGate_QkGate_XXMinusYY => StandardGate::XXMinusYY,
            QkGate_QkGate_XXPlusYY => StandardGate::XXPlusYY,
            QkGate_QkGate_CCX => StandardGate::CCX,
            QkGate_QkGate_CCZ => StandardGate::CCZ,
            QkGate_QkGate_CSwap => StandardGate::CSwap,
            QkGate_QkGate_RCCX => StandardGate::RCCX,
            QkGate_QkGate_C3X => StandardGate::C3X,
            QkGate_QkGate_C3SX => StandardGate::C3SX,
            QkGate_QkGate_RC3X => StandardGate::RC3X,
            _ => panic!("invalid value"),
        }
    }
}

impl StandardGate {
    // Returns the number of qubits supported by this gate
    pub fn num_qubits(&self) -> u32 {
        unsafe { qk_gate_num_qubits(*self as u8) }
    }
    // Returns the number of qubits supported by this gate
    pub fn num_params(&self) -> u32 {
        unsafe { qk_gate_num_params(*self as u8) }
    }
}
