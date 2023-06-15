import React,{useState} from "react";
import {CgProfile} from 'react-icons/cg';
//import logo1 from '../images/log_1png.png';
import logo1 from '../images/React Icons_files/logo_2.png';


function TellerLogin(){
    const[tellData,setTellData]=useState({
    UserID:'',
    tellPass:''
    });

    const handleTelSubmit = (e) =>{
        e.preventDefault();
        console.log(tellData);
    }
    const handleTellChange = (e) =>{
        const{name,value} = e.target;
        setTellData((prevTellData)=>({
            ...prevTellData,[name]:value }
            ))
    }




    return(
        <>
        <div className="TLogin">
            <div className="LeftPane">
            <img src={logo1} alt="" height="200px" width="300px"/>
            </div>
            <div className="RightPane">
               <CgProfile className="TellerProfile"/><br />
               <span><b>Sign in to dashboard</b></span>
               <form onSubmit={handleTelSubmit}>
                <input type="text" name="UserID" value={tellData.UserID} placeholder="UserID" onChange={handleTellChange}
                className="Userid"/><br />

                <input type="password" name="tellPass" value={tellData.tellPass} placeholder="password" onChange={handleTellChange}
                className="TellPass"/><br />
                <div className="Tellbtn">
                       <button type="submit">SignIn</button>
                </div>  
                

               </form>
            </div>
        </div>
        </>
    )
}

export default TellerLogin;
