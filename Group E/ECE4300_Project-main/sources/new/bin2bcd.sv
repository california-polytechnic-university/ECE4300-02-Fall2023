`timescale 1ns / 1ps
//////////////////////////////////////////////////////////////////////////////////
// Company: 
// Engineer: 
// 
// Create Date: 02/12/2023 06:10:50 PM
// Design Name: 
// Module Name: bin2bcd
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


module bin2bcd(
    input logic [7:0] bin,
    output logic [11:0] bcd    
    );   
    
    assign bcd[11] = 'b0;
    assign bcd[10] = 'b0;
    assign bcd[0] = bin[0];
    
    logic [0:18] S;
    
    add_3 Add0 (
    .A({0,bin[7:5]}),
    .S(S[0:3])
    );
    
    add_3 Add1 (
    .A({S[1:3],bin[4]}),
    .S(S[4:7])
    );
    
    add_3 Add2 (
    .A({S[5:7],bin[3]}),
    .S(S[8:11])
    );
    
    add_3 Add3 (
    .A({0, S[0], S[4], S[8]}),
    .S({bcd[9], S[12:14]})
    );
    
    add_3 Add4 (
    .A({S[9:11],bin[2]}),
    .S(S[15:18])
    );
    
    add_3 Add5 (
    .A({S[12:14], S[15]}),
    .S(bcd[8:5])
    );
    
    add_3 Add6 (
    .A({S[16:18],bin[1]}),
    .S(bcd[4:1])
    );
    
endmodule
