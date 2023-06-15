import React from 'react';
import './table.css'
function Table(){

    return(
<div className="tableP">
    <table>
       <tr className="tr1">
        <th>Name</th>
        <th> AccountNo</th>
        <th> Action</th>
        <th> NationalID</th>
        
        <th>Status</th>
        <th>Approval</th>
       </tr>
       <tr >
        <td>Emmauel Richmond Adotey</td>
        <td>144003938</td>
        <td>Deposit</td>
        <td>GHA-122-2323</td>
      
       </tr>
       <tr >
        <td>Emmauel</td>
        <td>144003938</td>
        <td>Deposit</td>
        <td>GHA-122-2323</td>
       
       </tr>
       <tr >
        <td >Emmauel</td>
        <td >144003938</td>
        <td>Deposit</td>
        <td>GHA-122-2323</td>
        
       </tr>
       <tr >
        <td >Emmauel</td>
        <td>144003938</td>
        <td>Deposit</td>
        <td>GHA-122-2323</td>
        
       </tr>
    
    </table>

</div>
    )
}
export default Table;