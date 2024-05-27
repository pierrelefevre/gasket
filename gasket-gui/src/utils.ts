export const chopString = (str: string, len: number) => {
  if (str.length > len) {
    return str.slice(0, len) + "...";
  }
  return str;
};
