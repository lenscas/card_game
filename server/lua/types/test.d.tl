local record test
    record Example
            -- pure Methods
            example_method: function(Example, number):(number)
            -- Mutating Methods
            example_method_mut: function(Example, number, string):(string)
            -- Pure functions
            example_function: function({string}):({string}, number)
            -- Mutating Functions
            example_function_mut: function(boolean, Example):(boolean, Example)

    end
    record Example2
            -- pure Methods
            example_method: function(Example, number):(number)
            -- Mutating Methods
            example_method_mut: function(Example, number, string):(string)
            -- Pure functions
            example_function: function({string}):({string}, number)
            -- Mutating Functions
            example_function_mut: function(boolean, Example):(boolean, Example)

    end
end
return test