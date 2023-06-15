import React,{useState} from "react";
import logo1 from "./tellerDashboard/Timages/Tlogo1.png";
import {Link} from 'react-router-dom';
import './Cvalidation';
function Login(){
    const [formdata,setFormdata]=useState({
        AccountNo:'',
        Password:''

    });
    const [errors,setErrors] = useState({});
    const handleChange = (e) =>{
    const{name,value} = e.target;
    setFormdata((prevFormdata)=>({
        ...prevFormdata,[name]:value}));
    };
    const handleSubmit= (e) =>{
        e.preventDefault();
        setErrors(Validation(values))
        console.log(formdata);
    }
    
    return(
        <>
        <div className="main">
            <div className="sub-main">
            <div>
            <img src={logo1} alt="Logo" width="fit-content" height="80px"/>
            <h1>Login</h1>
           
            <form className="Lform" onSubmit={handleSubmit}>
                   
                    <input type="text" name="AccountNo" value={formdata.AccountNo} onChange={handleChange} className="AccountNo" placeholder="Account number" /> <br />
                    <input type="Password" name="Password" value={formdata.Password} onChange={handleChange} placeholder="password" className="Password"/>

                    <div className="btn">
                       <button className="Lbutton" type="submit">SignIn</button>
                    </div>
                    <p >Click below to join queue as guest:</p>
                    <Link to="/Guestlogin"><b className="Gdetailsb"> Guest details</b></Link> 
                    
            </form>
            
            </div>
            
            </div>
            
        </div>
        </>
    )
}
export default Login