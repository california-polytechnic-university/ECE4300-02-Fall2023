`timescale 1ns / 1ps
//////////////////////////////////////////////////////////////////////////////////
// Company: 
// Engineer: 
// 
// Create Date: 10/14/2023 12:00:07 PM
// Design Name: 
// Module Name: diffe_hellman_algo
// Project Name: 
// Target Devices: 
// Tool Versions: 
// Description: 
// 
// Dependencies: 
// 
// Revision:
// Revision 0.01 - File Created
// Additional Comments:
// 
//////////////////////////////////////////////////////////////////////////////////


module exponential_function 
#(parameter N = 32)(
    input logic [3: 0] exponent,
    input logic [3: 0] base,
    output logic [N - 1: 0] be //base^exponent
    );
    
    logic [N - 1: 0] temp;
    integer i;
    always_comb
    begin
        temp = 1;
        for(i = 1; i <= exponent; i++)
        begin
            temp = (temp * base);
        end
    end
    
    assign be = temp;
    
endmodule
