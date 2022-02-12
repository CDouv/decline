import React from "react";

export const Debug = ({ exportParameters }) => {
  return (
    <div>
      <button className="button" onClick={() => exportParameters()}>
        Debug
      </button>
    </div>
  );
};

export default Debug;
