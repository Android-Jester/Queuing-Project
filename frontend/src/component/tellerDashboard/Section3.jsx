import React from 'react';
import './Section.css';
import {MdTimelapse} from 'react-icons/md'
function Section3 (){
    

    return(
 <div className="widget">
     <div className='wleft'> <MdTimelapse className="S1_icon"/> </div>
     <div className='wright'><b>Average Service Time</b> <br /><br /> <p className='S_font'>00:05:40</p></div>            
 </div>

    )
}
export default Section3;