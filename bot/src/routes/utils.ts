export function prettyPrint(obj: Record<string, any>) {
  console.log(JSON.stringify(obj, null, 2));
}
