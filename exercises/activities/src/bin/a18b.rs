// Topic: Result & the question mark operator
//
// Requirements:
// * Determine if an employee can access a building using a digital keycard
// * Employees that can access the building are:
//   * Maintenance crews
//   * Marketing department employees
//   * Managers
// * Other employees that work at the company are:
//   * Line supervisors
//   * Kitchen staff
//   * Assembly technicians
// * Ensure that terminated employees cannot access the building
//   regardless of their position
//
// Notes:
// * Use an enum to represent all types of employees
// * Use a struct to store the employee type and whether they are
//   still employed
// * Use a function that returns a Result to determine if the employee
//   may enter the building
// * Print whether the employee may access the building
//   * Must use a function that utilizes the question mark operator to do this

use std::result;


#[derive(Debug)]
enum EmployeeCategories {
    MaintenanceCrew,
    MarketingDeptEmployees,
    Managers,
    LineSupervisors, // from here onwards people cannot access building with there access cards
    KitchenStaff,
    AssemblyTechnicians,
}

#[derive(Debug)]
enum EmployeeState{
    Active,
    Terminated
}

struct Employee {
    id: i32,
    employee_type: EmployeeCategories,
    state: EmployeeState, // Active or Terminated State
}

fn check_if_employee_is_eligible_to_enter(employee_instance: &Employee) -> Result<String, String> {
    println!(
        "employee_instance.employee_type : {:?}",
        employee_instance.employee_type
    );
    println!("employee_instance. state : {:?}", employee_instance.state);

    // // using if-else when employees were string
    // if (employee_instance.state == "Active".to_owned())
    //     && (employee_instance.employee_type == ("MaintenanceCrew".to_owned())
    //         || employee_instance.employee_type == ("MarketingDeptEmployees".to_owned())
    //         || employee_instance.employee_type == ("Managers".to_owned()))
    // {
    //     Ok("This employee is eligible for access in building".to_owned());
    // } else { 
    //     Err("This employee is NOT eligible for access in building".to_owned());
    // }

    // using trivial match without question mark operator
    // better not to use nested match, instead we are using match below in two stages
    match employee_instance.state {
        // &str == "Active" ==> convert to owned string // when using strings, in current ex we are using enums instead
        // "Active"=> Ok("This employee is a active employee".to_owned()),
        // "Terminated"=> Ok("This employee is a active employee".to_owned()),
        EmployeeState::Active => (),
        EmployeeState::Terminated => return Err("terminated_string".to_owned()),
    };

    match employee_instance.employee_type {
        EmployeeCategories::MaintenanceCrew => {
            return Ok("active_MaintenanceCrew_string".to_owned())
        },
        EmployeeCategories::MarketingDeptEmployees => {
            return Ok("active_MarketingDeptEmployees_string".to_owned())
        },
        EmployeeCategories::Managers => {
            return Ok("active_Managers_string".to_owned())
        },
        _ => {
            return Err("terminated_string".to_owned())
        }
    };

}

fn question_operator_result_return_type (employee_instance: Employee) -> Result<(),String>{
    match employee_instance.state {
        // &str == "Active" ==> convert to owned string // when using strings, in current ex we are using enums instead
        // "Active"=> Ok("This employee is a active employee".to_owned()),
        // "Terminated"=> Ok("This employee is a active employee".to_owned()),
        EmployeeState::Active => (),
        EmployeeState::Terminated => return Err("terminated_string".to_owned()),
    };

    match employee_instance.employee_type {
        EmployeeCategories::MaintenanceCrew => {
            return Ok(())
        },
        EmployeeCategories::MarketingDeptEmployees => {
            return Ok(())
        },
        EmployeeCategories::Managers => {
            return Ok(())
        },
        _ => {
            return Err("terminated_string".to_owned())
        }
    };

}
fn main() {
    let employee_instance_ben = Employee {
        id: 101,
        employee_type: EmployeeCategories::MaintenanceCrew,
        state: EmployeeState::Active,
    };

    // // to print results using trivial match method
    // let return_result = check_if_employee_is_eligible_to_enter(&employee_instance_ben);
    // match return_result {
    //     Result::Ok(some_string) => println!("{:?}", some_string),
    //     Result::Err(some_string) => println!("{:?}", some_string),
    // };

    // to print results without using match
    main_for_question_operator(employee_instance_ben);

}

fn main_for_question_operator (employee_instance_ben:Employee) -> Result<(),String>{
    let return_result_from_question_operator = question_operator_result_return_type(employee_instance_ben)?;
    println!("return_result_from_question_operator: {:?}", return_result_from_question_operator); // empty operator
    println!("Access is eligible");
    println!("This functionality can run only and only when above question mark operator checks confirmation for OK Result");
    println!("In case of ERR Result, this line will not be executed");
    Ok(())
}
