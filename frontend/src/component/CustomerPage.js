import React,{useState} from 'react';
import logo1 from '../images/log_1png.png';
import {CgProfile} from 'react-icons/cg';




function CustomerPage(){
const [cusData,setCusData]=useState({
 cusId:''
});
const [dropdownCus,setDropdownCus]=useState("Deposit");
const handleChangeDropCus=(event)=>{
 setDropdownCus(event.target.value);
}

const handleCusChange =(event)=>{
    const{name,value}=event.target;
    setCusData((prevCusData)=>({
        ...prevCusData,[name]:value
    }))

}
return(
    <>
        <div className="CusHeader">
         <img src={logo1} alt="Logo" width="fit-content" height="70px"/> <CgProfile className="profile"/> 
        </div><hr />
        <div className="CusBody">
            <h1><i>Hello! User1</i></h1>
            <h2>Select Action:</h2>
            <select value={dropdownCus} onChange={handleChangeDropCus} className="cusDrop">
                <option value="Deposit">Deposit</option>
                <option value="Withdrawal">Withdrawal</option>
                <option value="ForeignExchange">ForeignExchange</option>
                <optgroup label="BillPayment">
                <option value="SchoolFees">School Fees</option>
                <option value="Utilities">Utilities</option>
                </optgroup>
              </select> 
              <h2>National_ID:</h2>
          <input type="text" name="cusId" onChange={handleCusChange} className="ID_field" value={cusData.cusId} placeholder="Enter digits after GHA-"/> <br />
           <div className="btn">
                       <button className="Cusbutton" type="submit">Join Queue</button>
                    </div>
           
         

                    </div>
   
    </>
)
}

export default CustomerPage;