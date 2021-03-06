import Header from "./components/Header";
import Parameters from "./components/Parameters";
import Calculate from "./components/Calculate";
import Debug from "./components/Debug";
import AddSegment from "./components/AddSegment";
import DeleteSegment from "./components/DeleteSegment";
import SegmentType from "./components/SegmentType";

import "./App.css";
import { useState } from "react";
import { queries } from "@testing-library/react";
import { isCompositeComponent } from "react-dom/test-utils";

const axios = require("axios").default;

const App = () => {
  const [param, setParam] = useState("");
  const [paramError, setParamError] = useState("false");
  const [segments, setSegments] = useState([
    {
      product: "oil",
      segmentNumber: 1,
      forecastType: "exponential",
      parameters: [
        {
          text: "Initial Flow Rate",
          symbol: "qi",
          units: "bbl/d",
          calculate: false,
          input: undefined,
        },

        {
          text: "Final Flow Rate",
          symbol: "qf",
          units: "bbl/d",
          calculate: false,
          input: undefined,
        },
        {
          text: "Initial Decline Rate",
          symbol: "di",
          units: "nominal %/yr",
          calculate: false,
          input: undefined,
        },

        {
          text: "Final Decline Rate",
          symbol: "df",
          units: "nominal %/yr",
          calculate: false,
          input: undefined,
        },

        {
          text: "Segment Duration",
          symbol: "t",
          units: "years",
          calculate: false,
          input: undefined,
        },

        {
          text: "Segment Reserves",
          symbol: "np",
          units: "mbbl",
          calculate: false,
          input: undefined,
        },
        {
          text: "Decline Exponent",
          symbol: "b",
          units: "",
          calculate: false,
          input: undefined,
        },
      ],
    },
  ]);

  //Create a state to track whether segment has been checked/approved to go into calculator
  const [inputCheck, setInputCheck] = useState([false]);

  const updateInputs = (arr) => {
    //Copy original segments
    let copySegments = segments.map((seg) => {
      return { ...seg };
    });

    let newSegments = segments.map((seg) => {
      return { ...seg };
    });

    newSegments
      .filter(
        (seg) =>
          (countUnknowns(seg.segmentNumber)[0] === 3 &&
            seg.forecastType === "exponential") ||
          (countUnknowns(seg.segmentNumber)[0] === 4 &&
            seg.forecastType === "hyperbolic")
      )
      .map((seg, index) => {
        seg.parameters.map((param, paramIndex) => {
          param.input = arr[index].parameters[paramIndex];
        });
      });

    newSegments.map((seg) => {
      copySegments[seg.segmentNumber - 1] = seg;
    });

    return setSegments(copySegments);
  };

  const toggleChangeInput = (symbol, val, segmentNumber) => {
    //Determine parameters to reference
    let params = segments[segmentNumber - 1].parameters;
    //Copy parameter, change inputs
    let newParameters = params.map((parameter) => {
      if (parameter.symbol === symbol) {
        //check if value is a number

        if (isNaN(val.trim()) || val.trim().length === 0) {
          return { ...parameter, input: val };
        } else {
          return { ...parameter, input: parseFloat(val) };
        }
      } else {
        return parameter;
      }
    });

    //Copy segments, add in new parameters
    let newSegments = segments.map((seg) => {
      return { ...seg };
    });
    newSegments[segmentNumber - 1].parameters = newParameters;

    return setSegments(newSegments);
  };
  const changeSegmentType = (segmentNumber, selectedSegmentType) => {
    let segCopy = segments.map((seg) => {
      return { ...seg };
    });

    segCopy = segCopy.map((seg) => {
      if (seg.segmentNumber === segmentNumber) {
        let newParameters = seg.parameters.map((param) => {
          return { ...param, input: "" };
        });
        let newSegment = {
          ...seg,
          forecastType: selectedSegmentType,
          parameters: newParameters,
        };

        return newSegment;
      } else {
        return seg;
      }
    });
    return setSegments(segCopy);
  };
  const toggleCalculate = (symbol, segmentNumber) => {
    //Determine parameters to reference
    let params = segments[segmentNumber - 1].parameters;
    //Copy parameter, change calculate field
    let newParameters = params.map((parameter) => {
      if (parameter.symbol === symbol) {
        const newCalculate = !parameter.calculate;
        if (newCalculate) {
          return { ...parameter, calculate: newCalculate, input: undefined };
        } else {
          return { ...parameter, calculate: newCalculate, input: undefined };
        }
      } else {
        return parameter;
      }
    });
    //Copy segments, add in new parameters
    let newSegments = segments.map((seg) => {
      return { ...seg };
    });
    newSegments[segmentNumber - 1].parameters = newParameters;

    return setSegments(newSegments);
  };

  const toggleUnits = (symbol, segmentNumber) => {
    //Determine parameters to reference
    let params = segments[segmentNumber - 1].parameters;
    //Copy parameter, change units and convert value
    let newParameters = params.map((parameter) => {
      if (parameter.symbol === symbol) {
        if (parameter.symbol === "di" || parameter.symbol === "df") {
          if (typeof parameter.input === "number") {
            switch (parameter.units) {
              case "nominal %/yr":
                return { ...parameter, units: "secant effective %/yr" };
              case "secant effective %/yr":
                return { ...parameter, units: "nominal %/yr" };
            }
          } else {
            switch (parameter.units) {
              case "nominal %/yr":
                return { ...parameter, units: "secant effective %/yr" };
              case "secant effective %/yr":
                return { ...parameter, units: "nominal %/yr" };
            }
          }
        }

        //symbol == duration
        if (parameter.symbol === "t") {
          if (typeof parameter.input === "number") {
            switch (parameter.units) {
              case "years":
                let newDaysInput = parameter.input * 365;
                return { ...parameter, units: "days", input: newDaysInput };
              case "days":
                let newYearsInput = parameter.input / 365;
                return { ...parameter, units: "years", input: newYearsInput };
            }
          } else {
            switch (parameter.units) {
              case "years":
                return { ...parameter, units: "days" };
              case "days":
                return { ...parameter, units: "years" };
            }
          }
        }
        return parameter;
      } else {
        return parameter;
      }
    });

    //Copy segments, add in new parameters
    let newSegments = segments.map((seg) => {
      return { ...seg };
    });
    newSegments[segmentNumber - 1].parameters = newParameters;

    return setSegments(newSegments);
  };

  const copySegment = () => {
    //Copy original segments
    let segmentsCopy = segments.map((seg) => {
      return { ...seg };
    });

    //Copy the last object in the array
    let newSegment = { ...segments[segments.length - 1] };

    //Define new parameters, zero out the inputs
    let newParameters = newSegment.parameters.map((parameter) => {
      return { ...parameter, input: undefined, calculate: false };
    });
    //Set segmentNumber = prev obj segmentNumber + 1
    newSegment.segmentNumber = segments[segments.length - 1].segmentNumber + 1;

    //add new parameters to new segment
    newSegment.parameters = newParameters;

    segmentsCopy.push(newSegment);

    setSegments(segmentsCopy);

    //add another value to inputsCheck
    let inputCheckCopy = [...inputCheck];

    inputCheckCopy.push(false);

    setInputCheck(inputCheckCopy);
  };

  const deleteSegment = () => {
    //Copy original segments
    let segmentsCopy = segments.map((seg) => {
      return { ...seg };
    });

    let inputCheckCopy = [...inputCheck];

    if (segmentsCopy.length == 1) {
      return;
    } else {
      segmentsCopy.pop();
      inputCheckCopy.pop();
    }

    setSegments(segmentsCopy);
    setInputCheck(inputCheckCopy);
  };

  const countUnknowns = (segmentNumber) => {
    let knownsCount = 0;
    let unknownsCount = 0;

    //Determine parameters to reference
    let params = segments[segmentNumber - 1].parameters;

    params.map((parameter) => {
      if (
        parameter.calculate === true &&
        parameter.input !== undefined &&
        !isNaN(parameter.input) &&
        parameter.input != ""
      ) {
        knownsCount += 1;
      } else {
        unknownsCount += 1;
      }
    });

    return [knownsCount, unknownsCount];
  };

  //Goal of the function: take segments state and:
  //1. Filter out any segments that don't satisfy conditions to be solved
  //2. For applicable segments, clear out inputs for any paramater where param.calculate === false
  const exportParameters = () => {
    //Pulling out just the segments that satisfy conditions to be solved
    let segCopy = segments.filter(
      (seg) =>
        (countUnknowns(seg.segmentNumber)[0] === 3 &&
          seg.forecastType === "exponential") ||
        (countUnknowns(seg.segmentNumber)[0] === 4 &&
          seg.forecastType === "hyperbolic")
    );

    segCopy = segCopy.map((seg) => {
      let newParameters = seg.parameters.map((param) => {
        if (param.calculate === false) {
          return { ...param, input: undefined };
        } else {
          return { ...param };
        }
      });
      let newSeg = { ...seg, parameters: newParameters };

      return newSeg;
    });

    setSegments(segCopy);
    return segCopy;
  };

  const sendJSON = async () => {
    let url = "http://localhost:8000/solve";
    let data = exportParameters();
    let config = {
      headers: {
        "Content-Type": "application/json",
      },
    };
    console.log("this is the data", data);
    data = JSON.stringify(data);

    try {
      const resp = await axios.post(url, data, config);
      console.log("RETURN DATA", resp.data);
      updateInputs(resp.data);
    } catch (err) {
      console.error(err);
    }
  };

  return (
    <div className="container">
      <Header title="Decline Calculator" />
      <div className="forecast">
        {segments.map((segment, index) => (
          <div
            className={`${
              countUnknowns(segment.segmentNumber)[0] === 3
                ? "segment"
                : "segmentError"
            }`}
          >
            <SegmentType
              changeSegmentType={changeSegmentType}
              segmentNumber={segments[index].segmentNumber}
            />
            <Parameters
              key={segments[index].segmentNumber}
              parameters={segments[index].parameters}
              changeInput={toggleChangeInput}
              onToggle={toggleCalculate}
              segmentNumber={segments[index].segmentNumber}
              segment={segments[index]}
              toggleUnits={toggleUnits}
              countUnknowns={countUnknowns}
            />
          </div>
        ))}
      </div>
      <div className="buttons">
        <AddSegment copySegment={copySegment} />
        <DeleteSegment deleteSegment={deleteSegment} />
        <Calculate
          countUnknowns={countUnknowns}
          exportParameters={exportParameters}
          sendJSON={sendJSON}
        />
        <Debug exportParameters={exportParameters} />
      </div>
    </div>
  );
};

export default App;
