import React, {useState} from 'react';
import './table.css'
function Table(){
    
    return(
<div className="tableP">
    <table>
        <tbody>
       <tr className="tr1">
        <th>Name</th>
        <th> AccountNo</th>
        <th> Action</th>
        <th> NationalID</th>
        
        <th>Status</th>
        <th>Approval</th>
       </tr>
       <tr >
        <td>Emmauel Richmond Adotey Kwesi</td>
        <td>144003938</td>
        <td>Deposit</td>
        <td>GHA-1221234565-9</td>
        <td>none</td>
        <td>
      <button className='S_complete'> complete</button> <button className='S_cancel' > Cancel</button>
        </td>      
       </tr>
       <tr >
        <td>Joseph K. Fosu</td>
        <td>144003938</td>
        <td>Deposit</td>
        <td>GHA-122-2323</td>
        <td>none</td>
        <td>
        <button className='S_complete'> complete</button> <button className='S_cancel' > Cancel</button>
        </td>
       </tr>
       <tr >
        <td >Henry K.</td>
        <td >144003938</td>
        <td>Deposit</td>
        <td>GHA-122-2323</td>
        <td>none</td>
        <td>
        <button className='S_complete'> complete</button> <button className='S_cancel' > Cancel</button> 
        </td>
       </tr>
       <tr >
        <td >Emmauella Sarp</td>
        <td>144003938</td>
        <td>Deposit</td>
        <td>GHA-122-2323</td>
         <td>none</td>
         <td>
         <button className='S_complete'> complete</button> <button className='S_cancel'> Cancel</button>
         </td>
       </tr>

       </tbody>
    </table>

</div>
    )
}
export default Table;