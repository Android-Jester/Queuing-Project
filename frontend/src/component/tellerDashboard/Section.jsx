import React from 'react';
import './Section.css';
import {BsHourglassSplit} from 'react-icons/bs';
import {BiHourglass} from 'react-icons/bi';
function Section (props){


    return(
 <div className="widget">
     <div className='wleft'><BsHourglassSplit className="S1_icon"/></div>
     <div className='wright'> <b>{props.content}</b> <br /><br /> <p className="S_font">00:14:35</p></div>           
 </div>

    )
}
export default Section;