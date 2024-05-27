import { Box, CircularProgress, Typography } from "@mui/material";

const CircularProgressWithLabel = ({
  value,
  ...props
}: {
  value: number;
  props?: any;
}) => {
  return (
    <Box sx={{ position: "relative", display: "inline-flex" }}>
      <CircularProgress
        variant="determinate"
        color={value > 85 ? "error" : "secondary"}
        value={value}
        sx={{ background: "#121212", borderRadius: "50%" }}
        {...props}
      />
      <Box
        sx={{
          top: 0,
          left: 0,
          bottom: 0,
          right: 0,
          position: "absolute",
          display: "flex",
          alignItems: "center",
          justifyContent: "center",
        }}
      >
        <Typography
          variant="caption"
          component="div"
          sx={{ fontWeight: "bold", fontFamily: "monospace" }}
        >{`${Math.round(value)}`}</Typography>
      </Box>
    </Box>
  );
};

export default CircularProgressWithLabel;
