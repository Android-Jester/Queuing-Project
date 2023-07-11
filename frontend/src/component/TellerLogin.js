import axios from "axios";
import React,{useState} from "react";
import { useNavigate } from "react-router-dom";
import {CgProfile} from 'react-icons/cg';
//import logo1 from '../images/log_1png.png';
import logo1 from '../images/React Icons_files/logo_2.png';


function TellerLogin(){
    const[tellData,setTellData]=useState({
    UserID:'',
    tellPass:''
    });
 const nav = useNavigate();

    const [is_Valid, setIs_Valid] = useState(true);
    const validateTellalogin= () =>{
        const regex3= /^\d{8}$/;
        const regex4 =  /^(?=.*\d)(?=.*[a-z])(?=.*[A-Z]).{8,}$/;

        if(tellData.UserID === "" || tellData.tellPass === ""){
            alert("UserID and Password cannot be empty");
            return;

        }if(regex3.test(tellData.UserID) && regex4.test(tellData.tellPass)){
            setIs_Valid(true);

        }else if(!(regex3.test(tellData.UserID))){
            alert("Invalid UserID");
            setIs_Valid(false);
        }else {
            alert("Invalid password format");
        }
    }


    const handleTelSubmit = (e) =>{
        e.preventDefault();
        validateTellalogin();
        if(is_Valid){
            axios.post(`${process.env.REACT_APP_BaseUrl}/teller/login`,{
                server_id: tellData.UserID,
                password:tellData.tellPass
              })
              .then(res => {
                  if(res.status === 200){
                    console.log(res.data) ;
                    nav('/Tella/Dashboard');
                  }

              }).catch(err =>{ 
                  console.log(err);
                  alert('No record Found')});

        }
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
                       <button className='T_button' type="submit">SignIn</button>
                </div>  
                

               </form>
            </div>
        </div>
        </>
    )
}

export default TellerLogin;
