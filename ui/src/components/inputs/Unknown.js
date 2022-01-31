//Will eventually use this component to talk to Rust and calculate unknown value
import React from "react";
import Parameter from "../Parameter";

export const Unknown = ({ parameter }) => {
  return (
    <div className="input">
      <form>
        <input type="text" value={parameter.input} readOnly="readOnly" />
      </form>
    </div>
  );
};

export default Unknown;
