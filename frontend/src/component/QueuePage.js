import React,{useState} from "react";
import logo1 from '../images/React Icons_files/logo_2.png';
import { CircularProgressbar } from 'react-circular-progressbar';
import 'react-circular-progressbar/dist/styles.css';


function QueuePage(){
 const [valueBar,setValueProgress]=useState('4');




    return(
        <>
        <div className="QueueHeader">
        <img src={logo1} alt="Logo" width="fit-content" height="80px"/>
        </div><br />
        <div className="QueueBody">
            <h1>Hi Steve</h1>
            <h2>Thank you for waiting</h2>
            <p>Your current Position in the queue  is:</p>    
        <div style={{ width: 80, height: 80 }}>
     <CircularProgressbar   minValue={1} maxValue={5} value={valueBar} text={valueBar} />
        </div>
        <p>Will make you aware when it is your turn</p>
        <div className="btn">
                       <button className="Qpbutton" type="submit">Leave queue </button>
        </div>
        </div>
        
        </>

    )
}
export default QueuePage;