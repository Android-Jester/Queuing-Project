import React from 'react';
import './Section.css';

import {AiOutlinePause} from 'react-icons/ai';
function Section2(){
    

    return(
 <div className="widget">
     <div className='wleft'> <AiOutlinePause className='S1_icon'/></div>
     <div className='wright'><b>Queue Length</b> <br /><br /> <p  className='S_font'>14</p> </div>            
 </div>

    )
}
export default Section2;