import { NextResponse } from "next/server";
import { getDatabase } from "../../lib/database";

export async function GET() {
  try {
    const db = await getDatabase();
    const escrows = await db.getAllEscrows();
    return NextResponse.json({
      success: true,
      data: escrows,
      count: escrows.length,
    });
  } catch (error) {
    console.error("Error fetching escrows:", error);
    return NextResponse.json(
      { error: "Failed to fetch escrows" },
      { status: 500 }
    );
  }
}
