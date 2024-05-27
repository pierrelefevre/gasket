import { useContext } from "react";
import { LbContext } from "../context/LbContext";

const useLb = () => useContext(LbContext);

export default useLb;
