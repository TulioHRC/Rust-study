pub enum OperationType {
    Sum,
    Subtract,
    Product,
    Division,
}

pub struct Operation {
    input_one: isize,
    input_two: isize,
    operation_type: OperationType,
}

impl Operation {
    pub fn create_operation(
        input_one: isize,
        input_two: isize,
        operation_type: OperationType,
    ) -> Self {
        return Operation {
            input_one,
            input_two,
            operation_type,
        };
    }

    pub fn display(&self) {
        println!(
            "\n{} {} {}", 
            self.input_one,
            match self.operation_type {
                OperationType::Sum => '+',
                OperationType::Subtract => '-',
                OperationType::Product => '*',
                OperationType::Division => '/',
            },
            self.input_two
        )
    }

    pub fn display_result(&self) {
        println!(
            "\n{} {} {} = {}", 
            self.input_one,
            match self.operation_type {
                OperationType::Sum => '+',
                OperationType::Subtract => '-',
                OperationType::Product => '*',
                OperationType::Division => '/',
            },
            self.input_two,
            self.execute()
        )
    }

    fn execute(&self) -> isize {
        return match self.operation_type {
            OperationType::Sum => self.input_one + self.input_two,
            OperationType::Subtract => self.input_one - self.input_two,
            OperationType::Product => self.input_one * self.input_two,
            OperationType::Division => self.input_one / self.input_two
        };
    }
}
