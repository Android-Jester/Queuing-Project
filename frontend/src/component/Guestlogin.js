import React,{useState} from "react";
import axios from 'axios';
import logo1 from "./tellerDashboard/Timages/Tlogo1.png";
function Guestlogin(){
    const[formdata2,setFormdata2]=useState({
        GuestFirstname:'',
        GuestLastname:'',
        TelephoneNo:'',
        IDcard:''
    });
    const handleChange1=(e)=>{
        const{name,value}=e.target;
        setFormdata2((prevFormdata2)=>({
            ...prevFormdata2,[name]:value})); }

            const handleSubmit1=(e)=>{
                e.preventDefault();
             
                    axios.get('http://10.42.0.1:3000/user/list', {withCredentials:true})
                    .then((response)=>{
                      console.log(response.data);
                    })
                    .catch((error)=>{
                      console.error(error);
                    })
               
            }
    const[dropdown,setDropdown]=useState("Deposit");
    const handleChangeDrop=(event)=>{
        setDropdown(event.target.value);

    }
    const[range,setRange]=useState("range1");
    const handleChangeRange=(event)=>{
        setRange(event.target.value);

    }

    return(
        <>
         <div className="main">
            <div className="sub-main1">
            <div>
            <img src={logo1} alt="Logo" width="fit-content" height="80px"/>
            <h1>Welcome Guest</h1>
            <form className="Gform"onSubmit={handleSubmit1}>
                <input type="text" name="GuestFirstname" value={formdata2.GuestFirstname} onChange={handleChange1} className="GuestFname" placeholder="First_Name"/> <br />
                <input type="text" name="GuestLastname" value={formdata2.GuestLastname} onChange={handleChange1} className="GuestLname" placeholder="Last_Name"/><br />

                <input type="tel" name="TelephoneNo" value={formdata2.TelephoneNo} onChange={handleChange1} className="TelephoneNo"placeholder="TelephoneNo"/><br />
                <input type="text" name="IDcard" value={formdata2.IDcard} onChange={handleChange1} className="IDcard"placeholder="National_ID"/> <br />
                <b><span className="Stext">Select an Action below:</span></b><br />
                <select value={dropdown} onChange={handleChangeDrop} className="Drop">
                  <option value="Deposit">Deposit</option>
                  <option value="Withdrawal">Withdrawal</option>
                  <option value="ForeignExchange">ForeignExchange</option>
                  <optgroup label="BillPayment">
                  <option value="SchoolFees">School Fees</option>
                  <option value="Utilities">Utilities</option>
                </optgroup>
              </select><br />
   
              
              <div className="btn">
                       <button className="Glbutton"type="submit">Join Queue</button>
                    </div>

      


            </form>
            </div>
            </div>

         </div>
         </>
        
    )
}
export default Guestlogin