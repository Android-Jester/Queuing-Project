import axios from "axios";
import React, { useContext, useState } from "react";
import { CircularProgressbar } from 'react-circular-progressbar';
import 'react-circular-progressbar/dist/styles.css';
import logo1 from '../images/React Icons_files/logo_2.png';
import { UserContext } from "./UserContext";


function QueuePage() {
    const [valueBar, setValueProgress] = useState('4');
    const { reValue, setReValue } = useContext(UserContext);

    const handleLeaveQueue = () => {
        axios.post(`${process.env.REACT_APP_BaseUrl}/user/leave`, {
            reValue
        }

        ).then(res => {

            console.log(res.data);


        }


        ).catch(err => {
            console.log(err);
        })
    }



    return (
        <>
            <div className="QueueHeader">
                <img src={logo1} alt="Logo" width="fit-content" height="80px" />
            </div><br />
            <div className="QueueBody">
                <h1>Hi {reValue.name}</h1>
                <h2>Thank you for waiting</h2>
                <p>Your current Position in the queue  is:</p>
                <div style={{ width: 80, height: 80 }}>
                    <CircularProgressbar minValue={1} maxValue={5} value={reValue.position} text={reValue.position} />
                </div>
                <p>Estimated waiting Time:</p>
                <p>00:23:56</p>
                <div className="btn">
                    <button className="Qpbutton" type="submit_Q" onClick={handleLeaveQueue}>Leave queue </button>
                </div>
            </div>

        </>

    )
}
export default QueuePage;