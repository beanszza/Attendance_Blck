#![no_std]
use soroban_sdk::{contract, contractimpl, contracttype, Address, Env, Symbol};

#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct AttendanceRecord {
    pub student: Address,
    pub date: u64,
    pub present: bool,
}

#[contract]
pub struct AttendanceContract;

const TEACHER: Symbol = Symbol::short("TEACHER");

#[contractimpl]
impl AttendanceContract {
    /// Initializes the contract with the teacher's address.
    /// The teacher is the only one authorized to mark attendance.
    pub fn init(env: Env, teacher: Address) {
        if env.storage().instance().has(&TEACHER) {
            panic!("already initialized");
        }
        env.storage().instance().set(&TEACHER, &teacher);
    }

    /// Marks a student as present or absent for a specific date.
    /// Maps to the MVP transaction: Teacher -> On-chain Action -> Result.
    pub fn mark_attendance(env: Env, student: Address, date: u64, present: bool) {
        let teacher: Address = env.storage().instance().get(&TEACHER).expect("not initialized");
        
        // Require authorization from the teacher
        teacher.require_auth();

        let key = (Symbol::short("RECORD"), student.clone(), date);
        
        // Ensure no duplicate entry for the same date
        if env.storage().persistent().has(&key) {
            panic!("attendance already marked for this date");
        }

        let record = AttendanceRecord {
            student: student.clone(),
            date,
            present,
        };

        // Store the attendance record on the ledger
        env.storage().persistent().set(&key, &record);
    }

    /// Retrieves the attendance record for a student on a specific date.
    pub fn get_attendance(env: Env, student: Address, date: u64) -> Option<AttendanceRecord> {
        let key = (Symbol::short("RECORD"), student, date);
        env.storage().persistent().get(&key)
    }
}

mod test;
