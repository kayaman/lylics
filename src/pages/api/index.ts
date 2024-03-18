// GET /
export async function GET() {
  console.log('Hello World');
  return new Response(JSON.stringify({ message: 'Hello World' }));
}
