import React,{useState, useEffect} from 'react';
import './Navbar.css';
import {CgProfile} from 'react-icons/cg';

function Navbar(){
    const [seconds, setSeconds] = useState(0);
 useEffect(() =>{
  const intervalId = setInterval(()=>{
    setSeconds((prevSeconds)=>prevSeconds + 1);
  }, 1000);
  return()=>{
    clearInterval(intervalId);
  };
 },[]);
 const formatTime = (seconds) =>{
    const minutes = Math.floor(seconds / 60);
    const remainingSeconds =seconds % 60;
    return `${minutes.toString().padStart(2, '0')}:${remainingSeconds.toString().padStart(2, '0')}`;
 };

    return(
        <div className ="navbar">
            <div className ="wrapper">
                TELLER ID:
 <h3>Timer:{formatTime(seconds)}</h3><CgProfile className="Tprofile"/>
            </div>
        </div>

    )
}
export default Navbar;